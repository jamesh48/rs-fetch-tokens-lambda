#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { RustAuthLambdaStack } from '../lib/rust-auth-lambda-stack';

const app = new cdk.App();
new RustAuthLambdaStack(app, 'RustAuthLambdaStack', {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  },
});
