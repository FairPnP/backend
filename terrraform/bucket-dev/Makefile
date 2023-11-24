TERRAFORM_VERSION := $(shell terraform --version | head -n 1 | sed -e 's/^Terraform v\(.*\)$$/\1/')
TERRAFORM_REQUIRED_VERSION := "1.5.2"
AWS_PROFILE_TO_USE := fairpnp

check:          ## Checks terraform version
	@echo "Checking terraform version... expecting version [${TERRAFORM_REQUIRED_VERSION}], found [${TERRAFORM_VERSION}]"
	@if [ "${TERRAFORM_VERSION}" != "${TERRAFORM_REQUIRED_VERSION}" ]; then echo "Please ensure you are running terraform ${TERRAFORM_VERSION}."; exit 1; fi

help:           ## Show this help. Ex. $ make help
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

init: check     ## Initialize terraform. Ex. $ make init
	@AWS_PROFILE=${AWS_PROFILE_TO_USE} terraform init --upgrade

validate: check ## Does a terraform validate. Ex. $ make validate
	@terraform validate

plan: check     ## run a plan against an env. Ex. $ make plan
	@echo "Will be planning the following to environment: ${AWS_PROFILE_TO_USE}"
	@terraform fmt
	@tflint --init
	@tflint --recursive
	@tfsec --exclude-downloaded-modules .
	@echo "Pulling the required modules..."
	@terraform get
	@terraform plan -out terraform-plan

apply: check    ## run an apply against an env. Ex. $ make apply
	@echo "Will be applying the following to environment: ${AWS_PROFILE_TO_USE}"
	@terraform show -no-color terraform-plan
	@terraform apply -auto-approve terraform-plan
	@test -f $(terraform-plan) && rm terraform-plan

clean:          ## Cleans up terraform files. Ex. $ make clean
	test -f $(terraform-plan*) && rm -f terraform-plan*
	test -d $(.terraform) && rm -rf .terraform
	test -f $(*.tfstate) && rm -f *.tfstate
	test -f $(.terraform.lock.hcl) && rm -f .terraform.lock.hcl

destroy: check  ## Destroys an environment. Ex. $ make destroy
	@echo "# 💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥 #"
	@echo "Are you really sure you want to completely destroy environment ?"
	@echo "# 💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥💥 #"
	@AWS_PROFILE=${AWS_PROFILE_TO_USE} terraform destroy
