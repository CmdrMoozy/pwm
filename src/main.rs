// Copyright 2015 Axel Rasmussen
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(
    anonymous_parameters,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces
)]
#![warn(bare_trait_objects, unreachable_pub, unused_qualifications)]

use flaggy::*;
use tracing_subscriber::{filter::LevelFilter, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(
                    if cfg!(debug_assertions) {
                        LevelFilter::DEBUG
                    } else {
                        LevelFilter::WARN
                    }
                    .into(),
                )
                .from_env()
                .unwrap(),
        )
        .init();

    main_impl(vec![
        pwm_lib::cli::build_config_command(),
        pwm_lib::cli::build_init_command(),
        pwm_lib::cli::build_addkey_command(),
        pwm_lib::cli::build_rmkey_command(),
        #[cfg(feature = "piv")]
        pwm_lib::piv::build_setuppiv_command(),
        #[cfg(feature = "piv")]
        pwm_lib::piv::build_addpiv_command(),
        #[cfg(feature = "piv")]
        pwm_lib::piv::build_rmpiv_command(),
        pwm_lib::cli::build_ls_command(),
        pwm_lib::cli::build_get_command(),
        pwm_lib::cli::build_set_command(),
        pwm_lib::cli::build_rm_command(),
        pwm_lib::cli::build_generate_command(),
        #[cfg(feature = "wifiqr")]
        pwm_lib::wifiqr::build_wifiqr_command(),
        pwm_lib::cli::build_export_command(),
        pwm_lib::cli::build_import_command(),
    ]);
}
