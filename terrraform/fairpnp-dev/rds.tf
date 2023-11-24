# ==============================================================================
# RDS Instance

# tfsec:ignore:aws-rds-enable-performance-insights-encryption
resource "aws_db_instance" "postgres" {
  identifier             = local.namespace
  db_name                = local.app_name
  allocated_storage      = 20
  storage_type           = "gp2"
  engine                 = "postgres"
  engine_version         = "13.12"
  instance_class         = "db.t3.micro"
  username               = "postgres"
  password               = "C0iWt3TiME8VnNMP6N0Y"
  parameter_group_name   = "default.postgres13"
  db_subnet_group_name   = module.vpc.database_subnet_group
  vpc_security_group_ids = [aws_security_group.rds_sg.id]
  skip_final_snapshot    = true

  # Backup settings - you can adjust these as needed
  backup_retention_period = 7
  backup_window           = "02:00-05:00"

  # Maintenance settings
  maintenance_window = "Sun:05:00-Sun:09:00"

  # Disable public accessibility for security
  publicly_accessible = false

  storage_encrypted                   = true
  iam_database_authentication_enabled = true
  deletion_protection                 = true

  # enable performance insights
  performance_insights_enabled = true

  tags = local.tags
}

# ==============================================================================
# Security Group

resource "aws_security_group" "rds_sg" {
  vpc_id      = module.vpc.vpc_id
  description = "rds security group"

  ingress {
    from_port       = 5432
    to_port         = 5432
    protocol        = "tcp"
    security_groups = [aws_security_group.ec2_sg.id] # Allowing access from the EC2 instance
    description     = "Allow PostgreSQL access from api-server security group"
  }

  tags = local.tags
}
