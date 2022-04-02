#!/bin/bash

base="http://localhost:6969"

echo "New User: "'curl $base/user/new'
echo
result=$(curl -s $base/user/new)
echo "$result"
echo

identity=$(echo "$result" | sed -r 's/.*identity":"([^"]+).*/\1/')
# echo "Identity: $identity"
# echo

echo "Bad command"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["a", "b"]}')
echo "$result"
echo

echo "Login @sam"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["login", "sam", "1234"]}')
echo "$result"
echo

echo "Ls"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["ls"]}')
echo "$result"
echo

echo "cd"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["cd", "common"]}')
echo "$result"
echo

echo "who"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["who"]}')
echo "$result"
echo

echo "Login @ron"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["login", "ron", "4321"]}')
echo "$result"
echo

echo "kill"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["kill", "sam"]}')
echo "$result"
echo

echo "self-kill"
echo
result=$(curl -s $base/query -H "Identity: $identity" -v -d '{"arguments": ["kill", "ron"]}')
echo "$result"
echo

echo "Status"
echo
result=$(curl -s $base/user/me -H "Identity: $identity" -v)
echo "$result"
echo
