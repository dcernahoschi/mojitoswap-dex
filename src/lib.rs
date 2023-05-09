use scrypto::prelude::*;

external_blueprint! {
    PoolPackageTarget {
        fn new(resource0_addr: ResourceAddress, resource1_addr: ResourceAddress, fee: Decimal,
            sqrt_price: Decimal, owner_badge_addr: ResourceAddress) -> ComponentAddress;
    }
}

#[blueprint]
mod dex_blueprint {
    struct Dex {
        dex_admin_badge_addr: ResourceAddress,
        pool_package_address: PackageAddress,
        pools: HashMap<ComponentAddress, (ResourceAddress, ResourceAddress)>,
    }

    impl Dex {
        pub fn new(pool_package_address: PackageAddress) -> (ComponentAddress, Bucket) {
            let dex_admin_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "DEX admin badge")
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);
            let auth_rules: AccessRulesConfig = AccessRulesConfig::new()
                .method(
                    "new_pool",
                    rule!(require(dex_admin_badge.resource_address())),
                    AccessRule::DenyAll,
                )
                .default(AccessRule::AllowAll, AccessRule::DenyAll);

            let dex = Self {
                dex_admin_badge_addr: dex_admin_badge.resource_address(),
                pool_package_address,
                pools: HashMap::new(),
            }
            .instantiate();
            (dex.globalize_with_access_rules(auth_rules), dex_admin_badge)
        }

        pub fn new_pool(
            &mut self,
            resource0_addr: ResourceAddress,
            resource1_addr: ResourceAddress,
            fee: Decimal,
            sqrt_price: Decimal,
            admin_auth: Proof,
        ) -> ComponentAddress {
            self.validate_admin_auth(admin_auth);
            let pool_addr = PoolPackageTarget::at(self.pool_package_address, "Pool").new(
                resource0_addr,
                resource1_addr,
                fee,
                sqrt_price,
                self.dex_admin_badge_addr,
            );
            self.pools.insert(pool_addr, (resource0_addr, resource1_addr));
            pool_addr
        }

        fn validate_admin_auth(&self, auth: Proof) -> ValidatedProof {
            auth.validate_proof(ProofValidationMode::ValidateContainsAmount(
                self.dex_admin_badge_addr,
                Decimal::one(),
            ))
            .expect("The provided admin badge is either of an invalid resource address or amount.")
        }
    }
}