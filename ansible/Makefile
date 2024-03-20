help: 
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

encrypt: 
	$(call check_defined, ENV, Please set the ENV to encrypt.)
	@ansible-vault encrypt environments/$(value ENV)/group_vars/all/env_vault.yml

decrypt:  
	$(call check_defined, ENV, Please set the ENV to decrypt.)
	@ansible-vault decrypt environments/$(value ENV)/group_vars/all/env_vault.yml

api-server-setup:
	$(call check_defined, ENV, Please set the ENV to plan for.)
	@echo "Environment is: $(value ENV)"
	@ansible-playbook --diff -i environments/$(value ENV) --extra-vars "ENV=$(value ENV)" --extra-vars "@environments/cross_env_vars.yml" playbooks/api-server-setup.yml -u ec2-user

api-server-deploy:
	$(call check_defined, ENV, Please set the ENV to plan for.)
	@echo "Environment is: $(value ENV)"
	@ansible-playbook --diff -i environments/$(value ENV) --extra-vars "ENV=$(value ENV)" --extra-vars "@environments/cross_env_vars.yml" playbooks/api-server-deploy.yml -u ec2-user

lint-all:
	@ansible-lint playbooks/api-server-setup.yml
	P1=$!
	wait ${P1} 

# Check that given variables are set and all have non-empty values,
# die with an error otherwise.
#
# Params:
#   1. Variable name(s) to test.
#   2. (optional) Error message to print.
check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))
