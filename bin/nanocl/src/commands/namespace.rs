use nanocl_utils::io_error::{IoResult, FromIo};
use nanocld_client::NanocldClient;

use crate::utils;
use crate::models::{
  NamespaceArgs, NamespaceCommands, NamespaceOpts, NamespaceRow,
  NamespaceDeleteOpts, NamespaceListOpts,
};

async fn exec_namespace_ls(
  client: &NanocldClient,
  options: &NamespaceListOpts,
) -> IoResult<()> {
  let items = client.list_namespace().await?;
  let namespaces = items
    .into_iter()
    .map(NamespaceRow::from)
    .collect::<Vec<NamespaceRow>>();

  match options.quiet {
    true => {
      for namespace in namespaces {
        println!("{}", namespace.name);
      }
    }
    false => {
      utils::print::print_table(namespaces);
    }
  }
  Ok(())
}

async fn exec_namespace_create(
  client: &NanocldClient,
  options: &NamespaceOpts,
) -> IoResult<()> {
  let item = client.create_namespace(&options.name).await?;
  println!("{}", item.name);
  Ok(())
}

async fn exec_namespace_inspect(
  client: &NanocldClient,
  options: &NamespaceOpts,
) -> IoResult<()> {
  let namespace = client.inspect_namespace(&options.name).await?;
  utils::print::print_yml(namespace)?;
  Ok(())
}

async fn exec_namespace_rm(
  client: &NanocldClient,
  options: &NamespaceDeleteOpts,
) -> IoResult<()> {
  if !options.skip_confirm {
    utils::dialog::confirm(&format!(
      "Delete namespace {}?",
      options.names.join(",")
    ))
    .map_err(|err| err.map_err_context(|| "Delete namespace"))?;
  }

  for name in &options.names {
    client.delete_namespace(name).await?;
  }

  Ok(())
}

pub async fn exec_namespace(
  client: &NanocldClient,
  args: &NamespaceArgs,
) -> IoResult<()> {
  match &args.commands {
    NamespaceCommands::List(options) => {
      exec_namespace_ls(client, options).await
    }
    NamespaceCommands::Create(options) => {
      exec_namespace_create(client, options).await
    }
    NamespaceCommands::Inspect(options) => {
      exec_namespace_inspect(client, options).await
    }
    NamespaceCommands::Remove(options) => {
      exec_namespace_rm(client, options).await
    }
  }
}
