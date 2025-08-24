#[cfg(test)]
pub mod corepc_node_helpers {
    use std::env;

    use bitcoin::{Address, BlockHash};
    use corepc_node::Node;

    use crate::{Auth, Client};

    /// Get the authentication credentials for a given `bitcoind` instance.
    fn get_auth(bitcoind: &Node) -> Auth {
        Auth::CookieFile(bitcoind.params.cookie_file.clone())
    }

    /// Mine a number of blocks of a given size `count`, which may be specified to a given coinbase
    /// `address`.
    pub fn mine_blocks(
        bitcoind: &Node,
        count: usize,
        address: Option<Address>,
    ) -> anyhow::Result<Vec<BlockHash>> {
        let coinbase_address = match address {
            Some(address) => address,
            None => bitcoind.client.new_address()?,
        };
        let block_hashes = bitcoind
            .client
            .generate_to_address(count as _, &coinbase_address)?
            .0
            .iter()
            .map(|hash| hash.parse::<BlockHash>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(block_hashes)
    }

    pub fn get_bitcoind_and_client() -> (Node, Client) {
        // setting the ENV variable `BITCOIN_XPRIV_RETRIEVABLE` to retrieve the xpriv
        unsafe {
            env::set_var("BITCOIN_XPRIV_RETRIEVABLE", "true");
        }
        let bitcoind = Node::from_downloaded().unwrap();

        let url = bitcoind.rpc_url();
        let client = Client::new(url, get_auth(&bitcoind), None, None).unwrap();
        (bitcoind, client)
    }
}
