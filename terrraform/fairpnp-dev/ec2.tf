locals {
  ec2_api_service = {
    num_instances    = 1
    iam_profile_role = aws_iam_role.assumable_roles["ec2_api_service"].name
    # === arm64
    # base image
    ami = "ami-06a869d0fb5f8ad84"
    # api-service image
    # ami           = "ami-07e6c81facba9cdb3"
    instance_type = "t4g.small"
    # === x86
    # ami           = "ami-089c26792dcb1fbd4"
    # instance_type = "t2.micro"
  }
}

# ==============================================================================
# EC2 Instance

resource "aws_instance" "api_server" {
  count = local.ec2_api_service.num_instances

  ami           = local.ec2_api_service.ami
  instance_type = local.ec2_api_service.instance_type

  subnet_id              = module.vpc.public_subnets[0]
  vpc_security_group_ids = [aws_security_group.ec2_sg.id]
  key_name               = aws_key_pair.deployer.key_name
  iam_instance_profile   = aws_iam_instance_profile.ec2_profile.name

  root_block_device {
    encrypted = true
  }

  metadata_options {
    http_tokens = "required"
  }

  tags = merge(local.tags, {
    Name = "${local.namespace}-api-server"
  })
}

resource "aws_iam_instance_profile" "ec2_profile" {
  name = "${local.namespace}-api-server"
  role = local.ec2_api_service.iam_profile_role
}

resource "aws_key_pair" "deployer" {
  key_name   = "deployer-key"
  public_key = file("../id_rsa_fairpnp.pub")
}

# ==============================================================================
# Security Group

#tfsec:ignore:aws-ec2-no-public-ingress-sgr
#tfsec:ignore:aws-ec2-no-public-egress-sgr
resource "aws_security_group" "ec2_sg" {
  vpc_id      = module.vpc.vpc_id
  description = "api-server security group"

  // SSH access
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [for ip in local.ip_allow_list : "${ip}/32"]
    description = "SSH access from allowed IPs"
  }

  // NLB access
  ingress {
    from_port   = 3000
    to_port     = 3000
    protocol    = "tcp"
    cidr_blocks = data.cloudflare_ip_ranges.current.ipv4_cidr_blocks
    description = "HTTP forwarded from NLB"
  }

  // Health check ingress
  ingress {
    from_port   = 3000
    to_port     = 3000
    protocol    = "tcp"
    cidr_blocks = [local.vpc.cidr]
    description = "Health checks from NLB"
  }

  // Standard egress rule to allow all outbound traffic
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    description = "Standard egress rule"
  }

  tags = local.tags
}

# ==============================================================================
# Outputs

output "api_server_public_ip" {
  value = {
    ips : [for i in aws_instance.api_server : i.public_ip]
  }
}
