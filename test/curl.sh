#!/bin/bash
set -e -o pipefail

export AWS_ACCESS_KEY_ID='123'
export AWS_SECRET_ACCESS_KEY='abc'
export S3_ENDPOINT='http://localhost:3000'
function LOG() {
    echo "*** $@"
}
function NL() {
    echo ""
}
function CURL() {
    LOG "curl $@"
    NL
    curl -s -i ${S3_ENDPOINT}$@
    NL
    NL
}

# setup
NL
CURL /
CURL /lala -X PUT
CURL /lala -I
CURL /
# test
CURL /lala -X GET
CURL /lala/README.md -X PUT -d @README.md
CURL /lala/README.md -X GET
# cleanup
CURL /lala -X DELETE
CURL /
