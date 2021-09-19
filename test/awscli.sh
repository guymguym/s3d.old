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
function S3() {
    LOG "aws s3 $@"
    NL
    aws --endpoint $S3_ENDPOINT s3 $@
    NL
}
function S3API() {
    LOG "aws s3api $@"
    NL
    aws --endpoint $S3_ENDPOINT s3api $@
    NL
}

# setup
NL
S3 ls
S3 mb s3://lala
S3API head-bucket --bucket lala
S3 ls
# test
S3 ls s3://lala
S3 cp README.md s3://lala/README.md
S3 cp s3://lala/README.md -
# cleanup
S3 rb s3://lala
S3 ls
