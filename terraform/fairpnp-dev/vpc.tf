locals {
  vpc = {
    name = "${local.namespace} VPC"
    cidr = "10.1.0.0/16"

    azs = {
      us-east-2a = {
        public_subnet   = "10.1.128.0/20"
        private_subnet  = "10.1.0.0/19"
        database_subnet = "10.1.176.0/24"
      },
      us-east-2b = {
        public_subnet   = "10.1.144.0/20"
        private_subnet  = "10.1.32.0/19"
        database_subnet = "10.1.177.0/24"
      },
      us-east-2c = {
        public_subnet   = "10.1.160.0/20"
        private_subnet  = "10.1.64.0/19"
        database_subnet = "10.1.178.0/24"
      },
    }
  }
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "5.0.0"

  name = local.vpc.name
  cidr = local.vpc.cidr

  azs              = keys(local.vpc.azs)
  public_subnets   = [for key, az in local.vpc.azs : az.public_subnet]
  private_subnets  = [for key, az in local.vpc.azs : az.private_subnet]
  database_subnets = [for key, az in local.vpc.azs : az.database_subnet]

  map_public_ip_on_launch = true

  enable_nat_gateway = false
  enable_vpn_gateway = false

  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = local.tags
}
