import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as iam from 'aws-cdk-lib/aws-iam';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';
import { Construct } from 'constructs';
import { LambdaTarget } from 'aws-cdk-lib/aws-elasticloadbalancingv2-targets';

export class RustAuthLambdaStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const lambdaFn = new lambda.Function(this, 'rsrg-tokens-lambda', {
      functionName: 'rust-fetch-tokens-lambda',
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(
        '../target/lambda/rs-fetch-tokens-lambda/bootstrap.zip'
      ),
      handler: 'provided',
      role: iam.Role.fromRoleArn(
        this,
        'imported-lambda-role',
        'arn:aws:iam::471507967541:role/jh-lambda-execution-role'
      ),
    });

    lambdaFn.grantInvoke(
      new iam.ServicePrincipal('elasticloadbalancing.amazonaws.com')
    );
    // const importedALBListener = elbv2.ApplicationListener.fromLookup(
    // this,
    // 'imported-listener',
    // {
    // listenerArn:
    // 'arn:aws:elasticloadbalancing:us-east-1:471507967541:listener/app/jh-alb/5927623bf7b387b8/202d118fecee2aa5',
    // }
    // );
    // const lambdaTarget = new LambdaTarget(lambdaFn);

    // const lambdaTargetGroup = new elbv2.ApplicationTargetGroup(
    //   this,
    //   'srg-rust-lambda-tg',
    //   {
    //     targets: [lambdaTarget],
    //   }
    // );

    // importedALBListener.addTargetGroups('srg-rust-tg', {
    //   targetGroups: [lambdaTargetGroup],
    //   priority: 19,
    //   conditions: [
    //     elbv2.ListenerCondition.hostHeaders(['data.stravareportgenerator.com']),
    //     elbv2.ListenerCondition.pathPatterns(['/srg-auth/tokens']),
    //   ],
    // });
  }
}
