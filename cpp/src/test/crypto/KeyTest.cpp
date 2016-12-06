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

#include <cstdint>
#include <string>
#include <vector>

#include "pwmc/crypto/Key.hpp"

namespace
{
struct KeyTestCase
{
	std::string passphrase;
	std::string salt;
	std::vector<uint8_t> expectedKey;

	KeyTestCase(std::string const &p, std::string const &s,
	            std::vector<uint8_t> const &ek)
	        : passphrase(p), salt(s), expectedKey(ek)
	{
	}
};

bool keyTestCasePasses(KeyTestCase const &test)
{
	pwm::crypto::Key key(test.passphrase, test.salt);
	if(std::vector<uint8_t>(
	           reinterpret_cast<uint8_t const *>(test.salt.data()),
	           reinterpret_cast<uint8_t const *>(test.salt.data()) +
	                   test.salt.length()) != key.getSalt())
	{
		return false;
	}
	return test.expectedKey == key.getKey();
}
}

TEST_CASE("Test cryptographic key derivation", "[Crypto]")
{
	const std::vector<KeyTestCase> TEST_CASES{
	        KeyTestCase("", "test",
	                    {0x19, 0x7c, 0x60, 0xe4, 0x38, 0xab, 0x4c, 0x8e,
	                     0xd6, 0xcb, 0x90, 0x4f, 0xed, 0x12, 0x86, 0xba,
	                     0xaa, 0x48, 0xea, 0x0b, 0x8b, 0x3c, 0x0d, 0xf8,
	                     0x43, 0xa4, 0x13, 0xd2, 0xb9, 0x3a, 0x65, 0x1a}),
	        KeyTestCase("password", "NaCl",
	                    {0x33, 0x40, 0x4c, 0xf8, 0xa3, 0x1c, 0xf5, 0xc5,
	                     0xa0, 0x94, 0x48, 0xb1, 0xbd, 0x11, 0xec, 0x4d,
	                     0x7e, 0xe1, 0x82, 0x75, 0x79, 0x2a, 0x79, 0x28,
	                     0x92, 0xde, 0x99, 0x98, 0xf0, 0x09, 0x34, 0xa6}),
	        KeyTestCase("pleaseletmein", "SodiumChloride",
	                    {0x0c, 0x7c, 0x76, 0x2d, 0x60, 0xc3, 0xd2, 0x98,
	                     0x10, 0xed, 0x10, 0x6a, 0xf2, 0xa9, 0x8e, 0x2c,
	                     0x9c, 0x60, 0x3e, 0xd8, 0xbe, 0xaa, 0xfe, 0x19,
	                     0x2c, 0x0f, 0x14, 0x7f, 0xad, 0xbd, 0x87, 0x57})};

	for(auto const &test : TEST_CASES)
		CHECK(keyTestCasePasses(test));
}
