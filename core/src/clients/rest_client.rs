use ureq::{Agent, AgentBuilder, Error};
use std::time::Duration;


pub fn rest_call(url :&str) ->  Result<String, Error> {

    println!("starting the REST call...\n");

  let agent: Agent = AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5))
      .build();

  let body: String = agent.get(url)
      .call()?
      .into_string()?;
    
    Ok(body)
}