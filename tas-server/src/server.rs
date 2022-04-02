//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use openapi_client::models;

use std::path::{Path, PathBuf};
use std::time::{Duration};
use std::io::{Write};
use std::thread;

use crate::members::{load_members, Members};
use crate::messages::{ClientMessage, ServerMessage};

use common::{
    Result,
    with_error_report,
    is_would_block_io_result,
    is_would_block_result,
};

use common::parsing::{self, read_message};
use common::shared::{IntoShared, Shared};
use common::shared::vec::{SharedVec};

use uuid::Uuid;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

struct UserData {
    name: String,
    location: PathBuf,
    identity: String,
}

type User = Shared<UserData>;

#[derive(Clone)]
struct SusContext {
    users: SharedVec<User>,
    members: Shared<Members>,
}

#[derive(Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
    context: SusContext,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{
            marker: PhantomData,
            context: SusContext {
                users: vec![].to_shared(),
                members: load_members().expect("Can't load members").to_shared(),
            },
        }
    }
}


use openapi_client::{
    Api,
    GetMyselfResponse,
    GetNewUserResponse,
    PostQueryResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

fn notify(message: &str) -> models::Notification {
    models::Notification {
        message: message.to_owned()
    }
}

fn get_my_data<C>(
    server: &Server<C>,
    context: &C
) -> std::result::Result<Option<User>, ApiError>
where
    C: Has<Option<swagger::AuthData>>,
{
    let maybe_auth = (context as &dyn Has<Option<swagger::AuthData>>).get();

    let auth = match maybe_auth {
        Some(it) => it,
        None => return Err("Generic failure".into()),
    };

    let sus = match auth {
        swagger::AuthData::ApiKey(key) => key,
        _ => return Err("Generic failure".into()),
    };

    let locked_users = match server.context.users.write() {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    for it in locked_users.iter() {
        let user_lock = match it.read() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        };

        if &user_lock.identity == sus {
            return Ok(Some(it.clone()));
        }
    }

    return Ok(None);
}

fn handle_login<C>(
    command: Vec<String>,
    server: &Server<C>,
    shared_me: User,
) -> std::result::Result<PostQueryResponse, ApiError> {
    if command.len() < 3 {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("The command misses some parameters")
        ))
    }

    let name = &command[1];
    let pass = &command[2];

    let members_lock = match server.context.members.read() {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    if !members_lock.has_user(name) {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify(&format!("No such a user > {}", name))
        ))
    }

    let settings = match members_lock.settings_for(name) {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    if pass != &settings.pass {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("Incorrect password")
        ))
    }

    match shared_me.write() {
        Ok(mut it) => {
            it.name = name.clone();
        }
        Err(error) => return Err(format!("{}", error).into())
    }

    let role = match members_lock.role_for(name) {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    let message = models::Role {
        title: role.title,
        allowed_commands: role.allowed_commands,
    };

    Ok(PostQueryResponse::HereIsANewRoleForYou(message))
}

fn handle_ls<C>(
    _command: Vec<String>,
    _server: &Server<C>,
    shared_me: User,
) -> std::result::Result<PostQueryResponse, ApiError> {
    let location = match shared_me.read() {
        Ok(it) => it.location.clone(),
        Err(error) => return Err(format!("{}", error).into())
    };

    let mut files = vec![];

    let contents = match std::fs::read_dir(location) {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    for it in contents {
        let that = match it {
            Ok(value) => value,
            Err(error) => return Err(format!("{}", error).into())
        };

        match that.path().to_str() {
            Some(thing) => files.push(thing.to_owned()),
            None => {}
        }
    }

    let message = models::FilesList { files };
    return Ok(PostQueryResponse::HereAreTheFiles(message))
}

fn location_to_string(location: &PathBuf) -> Result<String> {
    match location.to_str() {
        Some(thing) => Ok(thing.to_owned().replace("\\\\?\\", "")),
        None => Ok("Unknown".to_owned())
    }
}

fn handle_cd<C>(
    command: Vec<String>,
    _server: &Server<C>,
    shared_me: User,
) -> std::result::Result<PostQueryResponse, ApiError> {
    if command.len() < 2 {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("The command misses some parameters")
        ))
    }

    let target = &command[1];
    let target_path = Path::new(target);

    if !target_path.exists() {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("No such a path")
        ))
    }

    if !target_path.is_dir() {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("This is not a directory")
        ))
    }

    let mut new_location: PathBuf;

    if target_path.is_absolute() {
        new_location = target_path.to_path_buf();
    } else {
        new_location = match shared_me.read() {
            Ok(it) => it.location.clone(),
            Err(error) => return Err(format!("{}", error).into())
        };
        new_location.push(target);
    }

    let normalized = match new_location.canonicalize() {
        Ok(it) => it,
        Err(error) => {
            return Ok(PostQueryResponse::SomeRandomInformation(
                notify(&format!("Error > {:?}", error))
            ))
        }
    };

    match shared_me.write() {
        Ok(mut it) => {
            it.location = normalized.clone();
        }
        Err(error) => return Err(format!("{}", error).into())
    }

    let message = models::MoveTo {
        location: match location_to_string(&normalized) {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        }
    };

    Ok(PostQueryResponse::HereIsTheNewLocation(message))
}

fn collect_users<C>(
    server: &Server<C>,
) -> Result<Vec<models::UsersListUsers>> {
    let mut users = vec![];

    let users_lock = server.context.users.read()?;

    for it in users_lock.iter() {
        let name = it.read()?.name.clone();
        let location = it.read()?.location.clone();
        let string = location_to_string(&location)?;

        let data = models::UsersListUsers {
            user: name,
            location: string,
        };

        users.push(data);
    }

    Ok(users)
}

fn handle_who<C>(
    _command: Vec<String>,
    server: &Server<C>,
    _shared_me: User,
) -> std::result::Result<PostQueryResponse, ApiError> {
    let users = match collect_users(server) {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    let message = models::UsersList { users };

    Ok(PostQueryResponse::HereAreYourCrewmates(message))
}

fn are_locations_same(a: &PathBuf, b: &PathBuf) -> Result<bool> {
    let string_a = location_to_string(a)?;
    let string_b = location_to_string(b)?;
    Ok(string_a == string_b)
}

fn handle_kill<C>(
    command: Vec<String>,
    server: &Server<C>,
    shared_me: User,
) -> std::result::Result<PostQueryResponse, ApiError> {
    if command.len() < 2 {
        return Ok(PostQueryResponse::SomeRandomInformation(
            notify("The command misses some parameters")
        ))
    }

    let target = &command[1];

    let mut users_lock = match server.context.users.write() {
        Ok(it) => it,
        Err(error) => return Err(format!("{}", error).into())
    };

    let mut users_to_kill = vec![];

    let my_location = match shared_me.read() {
        Ok(it) => it.location.clone(),
        Err(error) => return Err(format!("{}", error).into())
    };

    for (index, raw_it) in users_lock.iter().enumerate() {
        let it = match raw_it.write() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        };

        let has_same_name = &it.name == target;
        let is_nearby = match are_locations_same(&it.location, &my_location) {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        };

        if has_same_name && is_nearby {
            users_to_kill.push(index);
        }
    }

    let mut shift = 0;

    for it in &users_to_kill {
        users_lock.remove(it - shift);
        shift += 1;
    }

    let message = models::KillResult {
        killed_users_count: users_to_kill.len() as u32,
    };

    Ok(PostQueryResponse::HereIsTheKillResult(message))
}

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Has<Option<swagger::Authorization>> + Has<Option<swagger::AuthData>> + Send + Sync
{
    /// Returns your status
    async fn get_myself(
        &self,
        context: &C) -> std::result::Result<GetMyselfResponse, ApiError>
    {
        let context = context.clone();

        let maybe_auth = (context as &dyn Has<Option<swagger::AuthData>>).get();

        let auth = match maybe_auth {
            Some(it) => it,
            None => return Err("Generic failure".into()),
        };

        let sus = match auth {
            swagger::AuthData::ApiKey(key) => key,
            _ => return Err("Generic failure".into()),
        };

        let locked_users = match self.context.users.write() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        };

        let mut is_alive = false;

        for it in locked_users.iter() {
            let user_lock = match it.read() {
                Ok(it) => it,
                Err(error) => return Err(format!("{}", error).into())
            };

            if &user_lock.identity == sus {
                is_alive = true;
                break
            }
        }

        let response = models::InlineResponse2001 {
            is_alive: is_alive,
        };

        Ok(GetMyselfResponse::HereIsYourStatus(response))
    }

    /// Get the initial user context
    async fn get_new_user(
        &self,
        _context: &C) -> std::result::Result<GetNewUserResponse, ApiError>
    {
        let the_members = match self.context.members.read() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let guest_role = match the_members.role_for("guest") {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let role_data = models::Role {
            title: guest_role.title,
            allowed_commands: guest_role.allowed_commands,
        };

        let directory = match std::env::current_dir() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let location = match directory.to_str() {
            Some(thing) => thing.to_owned(),
            None => return Err("Generic failure".into()),
        };

        let location_data = models::MoveTo {
            location: location,
        };

        let identity = Uuid::new_v4().to_hyphenated().to_string();

        let resposnse = models::InlineResponse200 {
            role: role_data,
            location: location_data,
            identity: identity.clone(),
        };

        let user = UserData {
            name: format!("guest"),
            location: directory,
            identity: identity,
        }.to_shared();

        let mut locked_users = match self.context.users.write() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into())
        };

        locked_users.push(user);

        Ok(GetNewUserResponse::HereIsTheDefaultIdentity(resposnse))
    }

    /// Run a command
    async fn post_query(
        &self,
        request_body: models::Query,
        context: &C) -> std::result::Result<PostQueryResponse, ApiError>
    {
        let context = context.clone();
        let command = &request_body.arguments;

        if command.len() == 0 {
            return Ok(PostQueryResponse::SomeRandomInformation(notify("Empty command")))
        }

        let shared_me = match get_my_data(self, context) {
            Ok(it) => match it {
                Some(that) => that,
                None => return Ok(PostQueryResponse::YouAreDead),
            },
            Err(error) => return Err(format!("{}", error).into()),
        };

        let name = match shared_me.read() {
            Ok(it) => it.name.clone(),
            Err(error) => return Err(format!("{}", error).into()),
        };

        let the_members = match self.context.members.read() {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into()),
        };

        let role = match the_members.role_for(&name) {
            Ok(it) => it,
            Err(error) => return Err(format!("{}", error).into()),
        };

        if !role.allowed_commands.contains(&command[0]) {
            return Ok(PostQueryResponse::SomeRandomInformation(notify("No such an allowed command for you")))
        }

        return match command[0].as_ref() as &str {
            "login" => handle_login(command.clone(), self, shared_me),
            "ls" => handle_ls(command.clone(), self, shared_me),
            "cd" => handle_cd(command.clone(), self, shared_me),
            "who" => handle_who(command.clone(), self, shared_me),
            "kill" => handle_kill(command.clone(), self, shared_me),
            it => Ok(PostQueryResponse::SomeRandomInformation(
                notify(&format!("No such a command > {}", it))
            ))
        }
    }

}
