terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0"
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

module "k8s" {
  source = "./k8s"

  k8s_host                   = azurerm_kubernetes_cluster.k8s.kube_config[0].host
  k8s_certificate            = base64decode(azurerm_kubernetes_cluster.k8s.kube_config[0].client_certificate)
  k8s_key                    = base64decode(azurerm_kubernetes_cluster.k8s.kube_config[0].client_key)
  k8s_cluster_ca_certificate = base64decode(azurerm_kubernetes_cluster.k8s.kube_config[0].cluster_ca_certificate)

  dns_prefix = var.dns_prefix
}
