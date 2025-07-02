# Deployment

This is the terraform code to deploy the infrastructure for the project.
The [aks.tf](aks.tf) file contains the configuration for the Azure Kubernetes Service (AKS) cluster.
All the kubernetes resources are defined in the [k8s](k8s) directory.
We use [ingress-nginx](k8s/nginx.tf) as the ingress controller, [cert-manager](k8s/cert-manager.tf) for certificate management, [postgresql](k8s/db.tf) for the database and [nats](k8s/nats.tf) for making multiple backend replicas share messages (all deployed as Helm charts).
The [app.tf](k8s/app.tf) file contains the configuration for the application, including the deployment, service, ingress, secret and namepspace resources.
The image tag of the application is set via a environment variable in the GitHub Actions workflow with is the current commit tag.

## Usage

To deploy the infrastructure, you need to have create a new release in GitHub with a tag that matches the version you want to deploy with the pattern `vX.Y.Z`.
