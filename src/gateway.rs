use pingora::Error;
use pingora::{proxy::Session, prelude::HttpPeer};
use pingora::proxy::ProxyHttp;
use async_trait::async_trait; 
use log::info;

pub struct MyGateway {
    
}  




#[async_trait]
impl ProxyHttp for MyGateway {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}


    async fn upstream_peer(
        &self,
        _: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>, Box<Error>> {
        let addr = ("127.0.0.1", 8080);
        let peer = Box::new(HttpPeer::new(addr, false, String::new()));

        Ok(peer)
    }


    //we always filter requests because there really isn't an upstream peer.
    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool, Box<Error>> {
        let _ = session.respond_error(403).await;
        return Ok(true)
    }

    async fn logging(&self, session: &mut Session, _e: Option<&pingora::Error>, _: &mut Self::CTX,) {
        let _ =  session.req_header().uri.path().split('/').skip_while(|x| x.is_empty()).next().and_then(|element| {
          info!("root: {}", element);
          Some(())
        });

        
    }    
    
}
