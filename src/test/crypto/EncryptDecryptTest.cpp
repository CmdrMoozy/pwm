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

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "pwmc/crypto/decrypt.hpp"
#include "pwmc/crypto/encrypt.hpp"
#include "pwmc/crypto/generatePassword.hpp"
#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/Util.hpp"

namespace
{
constexpr std::size_t ENCRYPTION_TEST_DATA_SIZE_BYTES = 4096;
}

TEST_CASE("Encryption round trip test", "[Crypto]")
{
	std::string password = pwm::crypto::generatePassword();
	pwm::crypto::Key key(password);
	std::vector<uint8_t> plaintext = pwm::crypto::util::generateRandomBytes(
	        ENCRYPTION_TEST_DATA_SIZE_BYTES,
	        pwm::crypto::util::RandomQuality::WEAK);

	std::vector<uint8_t> encrypted = pwm::crypto::encrypt(key, plaintext);

	std::vector<uint8_t> decrypted = pwm::crypto::decrypt(key, encrypted);

	CHECK(plaintext == decrypted);
	CHECK(plaintext != encrypted);
	CHECK(encrypted.size() ==
	      (plaintext.size() + 2 * pwm::crypto::DEFAULT_IV_SIZE_OCTETS));
}
