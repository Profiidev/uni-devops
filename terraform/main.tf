terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
  required_version = ">= 1.1.0"

  cloud {
    organization = "dealing-with-devops"

    workspaces {
      name = "dealing-with-devops"
    }
  }
}

provider "azurerm" {
  features {}
}
