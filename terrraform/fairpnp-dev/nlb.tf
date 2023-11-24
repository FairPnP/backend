locals {
  nlb = {
    name              = "${local.namespace}-nlb"
    target_group_name = "${local.namespace}-tg"
  }
}
// ==============================================================================
// NLB

#tfsec:ignore:aws-elb-alb-not-public
resource "aws_lb" "api_nlb" {
  name                             = local.nlb.name
  internal                         = false
  load_balancer_type               = "network"
  subnets                          = module.vpc.public_subnets
  enable_cross_zone_load_balancing = true

  tags = local.tags
}

resource "aws_lb_target_group" "api_nlb" {
  name     = local.nlb.target_group_name
  port     = 3000
  protocol = "TCP"
  vpc_id   = module.vpc.vpc_id

  health_check {
    protocol            = "HTTP"
    path                = "/health"
    port                = "3000"
    matcher             = "200"
    interval            = 30
    healthy_threshold   = 2
    unhealthy_threshold = 2
    timeout             = 5
  }
}

resource "aws_lb_target_group_attachment" "api_lb" {
  count = local.ec2_api_service.num_instances

  target_group_arn = aws_lb_target_group.api_nlb.arn
  target_id        = aws_instance.api_server[count.index].id
  port             = 3000
}

resource "aws_lb_listener" "api_tls" {
  load_balancer_arn = aws_lb.api_nlb.arn
  port              = 443
  protocol          = "TLS"
  ssl_policy        = "ELBSecurityPolicy-TLS-1-2-2017-01"
  certificate_arn   = aws_acm_certificate.api_cert.arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.api_nlb.arn
  }

  depends_on = [aws_acm_certificate.api_cert]
}

// ==============================================================================
// ACM

resource "aws_acm_certificate" "api_cert" {
  domain_name       = local.api_full_domain
  validation_method = "DNS"

  tags = local.tags
}
