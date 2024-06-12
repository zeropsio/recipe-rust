# Zerops x Rust
This is the most bare-bones example of Rust app running on Zerops.

![rust](https://github.com/zeropsio/recipe-shared-assets/blob/main/covers/cover-rust.png)

## Deploy on Zerops
You can either click the deploy button to deploy directly on Zerops, or manually copy the [import yaml](https://github.com/zeropsio/recipe-rust/blob/main/zerops-project-import.yml) to the import dialog in the Zerops app.

<a href="https://app.zerops.io/recipe/rust">
    <img width="250" alt="Deploy on Zerops" src="https://github.com/zeropsio/recipe-shared-assets/blob/main/deploy-button/deploy-button.png">
</a>

<br/>
<br/>

## Recipe features
- **Rust 1.76** on **Zerops Rust** service
- Zerops **PostgreSQL 16** service as database
- Healthcheck setup example
- Utilization of Zerops' built-in **environment variables** system
- Utilization of Zerops' built-in **log management**

## Production vs. development

Base of the recipe is ready for production, the difference comes down to:

- Use highly available version of the PostgreSQL database (change *mode* from *NON_HA* to *HA* in recipe YAML, *db* service section)
- Use at least two containers for the Rust service to achieve high reliability and resilience (add *minContainers: 2* in recipe YAML, *api* service section)

Further things to think about when running more complex, highly available Rust production apps on Zerops:

- Containers are volatile - use Zerops object storage to store your files
- Use Zerops Redis (KeyDB) for caching, storing sessions and pub/sub messaging
- Use more advanced logging lib, such as [slog](https://github.com/slog-rs/slog)