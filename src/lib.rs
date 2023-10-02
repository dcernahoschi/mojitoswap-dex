use scrypto::prelude::*;

#[blueprint]
mod dex_blueprint {

    extern_blueprint! {
        "package_tdx_2_1pkmkzk8w82r064prwecrqrjrfrp88jhdwqkyv49juek4ynhn7476xx",
        Pool {
            fn new(resource0_addr: ResourceAddress, resource1_addr: ResourceAddress, fee: Decimal,
                sqrt_price: Decimal, owner_badge_addr: ResourceAddress, low_sqrt_price: Decimal, high_sqrt_price: Decimal,
                bucket0: Bucket, bucket1: Bucket) -> (Global<Pool>, Bucket, Bucket, Bucket);
        }
    }

    enable_method_auth! {
        roles {
            admin => updatable_by: [];
        },
        methods {
            new_pool => restrict_to: [admin];
        }
    }

    struct Dex {
        dex_admin_badge_addr: ResourceAddress,
        pools: HashSet<ComponentAddress>,
    }

    impl Dex {
        pub fn new() -> (Global<Dex>, Bucket) {
            let dex_admin_badge: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1)
                .into();

            let dex = Self {
                dex_admin_badge_addr: dex_admin_badge.resource_address(),
                pools: HashSet::new(),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .roles(roles!(
                admin => rule!(require(dex_admin_badge.resource_address()));
            ))
            .globalize();
            (dex, dex_admin_badge)
        }

        pub fn new_pool(
            &mut self,
            resource0_addr: ResourceAddress,
            resource1_addr: ResourceAddress,
            fee: Decimal,
            sqrt_price: Decimal,
            low_sqrt_price: Decimal,
            high_sqrt_price: Decimal,
            bucket0: Bucket,
            bucket1: Bucket,
        ) -> (ComponentAddress, Bucket, Bucket, Bucket) {
            let (pool, pos_nft, rmd_bucket0, rmd_bucket1) = Blueprint::<Pool>::new(
                resource0_addr,
                resource1_addr,
                fee,
                sqrt_price,
                self.dex_admin_badge_addr,
                low_sqrt_price,
                high_sqrt_price,
                bucket0,
                bucket1,
            );
            self.pools.insert(pool.address());
            (pool.address(), pos_nft, rmd_bucket0, rmd_bucket1)
        }
    }
}
