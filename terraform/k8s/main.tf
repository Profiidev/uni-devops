terraform {
  required_providers {
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.0"
    }
    kubectl = {
      source  = "gavinbunney/kubectl"
      version = "~> 1.0"
    }
  }
  required_version = ">= 1.1.0"
}

provider "kubernetes" {
  host                   = var.k8s_host
  client_certificate     = var.k8s_certificate
  client_key             = var.k8s_key
  cluster_ca_certificate = var.k8s_cluster_ca_certificate
}

provider "helm" {
  kubernetes {
    host                   = var.k8s_host
    client_certificate     = var.k8s_certificate
    client_key             = var.k8s_key
    cluster_ca_certificate = var.k8s_cluster_ca_certificate
  }
}

provider "kubectl" {
  host                   = var.k8s_host
  client_certificate     = var.k8s_certificate
  client_key             = var.k8s_key
  cluster_ca_certificate = var.k8s_cluster_ca_certificate
}
