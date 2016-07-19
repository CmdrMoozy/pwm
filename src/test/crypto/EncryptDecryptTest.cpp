/*
 * pwm - A simple password manager for Linux.
 * Copyright (C) 2015  Axel Rasmussen
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <catch/catch.hpp>

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/Util.hpp"
#include "pwmc/crypto/decrypt.hpp"
#include "pwmc/crypto/encrypt.hpp"
#include "pwmc/crypto/generatePassword.hpp"

TEST_CASE("Encryption round trip test", "[Crypto]")
{
	static const std::vector<std::size_t> TEST_CASE_DATA_SIZES{4096, 123};

	for(std::size_t dataSize : TEST_CASE_DATA_SIZES)
	{
		std::string password = pwm::crypto::generatePassword();
		pwm::crypto::Key key(password);
		std::vector<uint8_t> plaintext =
		        pwm::crypto::util::generateRandomBytes(
		                dataSize,
		                pwm::crypto::util::RandomQuality::WEAK);

		std::vector<uint8_t> encrypted =
		        pwm::crypto::encrypt(key, plaintext);
		std::vector<uint8_t> decrypted =
		        pwm::crypto::decrypt(key, encrypted);

		CHECK(plaintext == decrypted);
		CHECK(plaintext != encrypted);
	}
}
