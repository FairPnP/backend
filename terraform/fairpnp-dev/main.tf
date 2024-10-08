locals {
  app_name = "fairpnp"
  env      = "dev"
  # name is used for general naming of resources
  namespace = "${local.app_name}-${local.env}"

  ip_allow_list = ["99.250.145.3", "209.171.85.227"]

  # AWS profile name
  profile = "fairpnp"

  region = "us-east-2"

  tags = {
    name        = local.app_name
    environment = local.env
    terraform   = "true"
  }

}
