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

use bdrck::flags::*;

fn main() {
    let debug: bool = cfg!(debug_assertions);
    bdrck::logging::init(
        bdrck::logging::OptionsBuilder::new()
            .set_filters(match debug {
                false => "warn".parse().unwrap(),
                true => "debug".parse().unwrap(),
            })
            .set_panic_on_output_failure(debug)
            .set_always_flush(true)
            .build()
            .unwrap(),
    );

    main_impl_multiple_commands(vec![
        pwm_lib::cli::build_config_command(),
        pwm_lib::cli::build_init_command(),
        pwm_lib::cli::build_addkey_command(),
        pwm_lib::cli::build_rmkey_command(),
        #[cfg(feature = "piv")]
        pwm_lib::piv::build_setuppiv_command(),
        #[cfg(feature = "piv")]
        pwm_lib::piv::build_addpiv_command(),
        pwm_lib::cli::build_ls_command(),
        pwm_lib::cli::build_get_command(),
        pwm_lib::cli::build_set_command(),
        pwm_lib::cli::build_rm_command(),
        pwm_lib::cli::build_generate_command(),
        pwm_lib::cli::build_export_command(),
        pwm_lib::cli::build_import_command(),
    ]);
}
