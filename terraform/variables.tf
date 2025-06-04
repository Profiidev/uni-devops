variable "location" {
  type        = string
  description = "The Azure region where the resources will be created."
  default     = "westeurope"
}

variable "name" {
  type        = string
  description = "The name of the Azure Kubernetes Service (AKS) cluster."
  default     = "dealing-with-devops-aks"
}

variable "resource_group_name" {
  type        = string
  description = "The name of the resource group where the AKS cluster will be created."
  default     = "rg-buerger-larisch"
}

variable "username" {
  type        = string
  description = "The username for the AKS cluster."
  default     = "devops"
}

variable "dns_prefix" {
  type        = string
  description = "The DNS prefix for the AKS cluster."
  default     = "dealing-with-devops"
}
