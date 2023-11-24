
# Bootstrapping
### State bucket

We run the bucket project this will create an s3 bucket. We can give that state to this terraform project.
By keeping this seperate even if we run a terraform destroy we don't remove our bucket as this breaks terraform.

### Important DNS

The hosted zone is created MANUALLY, this is due to the fact we do NOT control the origin of the domain.
As such we have to pass them the NS records ahead of time!

<!-- BEGIN_TF_DOCS -->
<!-- END_TF_DOCS -->
