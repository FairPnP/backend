locals {
  app_name = "fairpnp"
  env      = "dev"
  # name is used for general naming of resources
  namespace = "${local.app_name}-${local.env}"

  ip_allow_list = ["99.250.145.3", "189.174.72.164"]

  # AWS profile name
  profile = "fairpnp"

  region = "us-east-2"

  tags = {
    name        = local.app_name
    environment = local.env
    terraform   = "true"
  }

}
