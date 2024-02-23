# ==============================================================================
# ElastiCache Redis Instance

resource "aws_elasticache_cluster" "redis_cluster" {
  cluster_id           = "${local.namespace}-redis"
  engine               = "redis"
  engine_version       = "7.1"
  node_type            = "cache.t3.micro"
  num_cache_nodes      = 1
  parameter_group_name = "default.redis7"
  subnet_group_name    = aws_elasticache_subnet_group.redis_subnet_group.name

  # Security settings
  security_group_ids = [aws_security_group.redis_sg.id]

  # Disable public accessibility for security
  port = 6379

  # Snapshot settings
  snapshot_retention_limit = 7
  snapshot_window          = "05:00-06:00"

  # Maintenance settings
  maintenance_window = "sun:06:00-sun:08:00"

  tags = local.tags
}

# Subnet Group for Redis
resource "aws_elasticache_subnet_group" "redis_subnet_group" {
  name       = "${local.namespace}-redis-subnet-group"
  subnet_ids = module.vpc.database_subnets

  tags = local.tags
}

# ==============================================================================
# Security Group for Redis

resource "aws_security_group" "redis_sg" {
  vpc_id      = module.vpc.vpc_id
  description = "Redis security group"

  ingress {
    from_port       = 6379
    to_port         = 6379
    protocol        = "tcp"
    security_groups = [aws_security_group.ec2_sg.id] # Allowing access from the EC2 instance
    description     = "Allow Redis access from api-server security group"
  }

  tags = local.tags
}

