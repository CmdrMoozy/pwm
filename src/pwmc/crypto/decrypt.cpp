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

#include "decrypt.hpp"

#include <gcrypt.h>

#include <bdrck/util/ScopeExit.hpp>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/checkReturn.hpp"

namespace
{
std::vector<uint8_t> decryptImpl(pwm::crypto::Key const &key, int algorithm,
                                 std::vector<uint8_t> const &ciphertext)
{
	if(ciphertext.size() <= pwm::crypto::DEFAULT_IV_SIZE_OCTETS)
		return std::vector<uint8_t>();

	gcry_cipher_hd_t cipher;
	pwm::crypto::checkReturn(gcry_cipher_open(
	        &cipher, algorithm, GCRY_CIPHER_MODE_CBC, GCRY_CIPHER_SECURE));
	bdrck::util::ScopeExit destroyCipher(
	        [&cipher]() { gcry_cipher_close(cipher); });

	pwm::crypto::checkReturn(
	        gcry_cipher_setiv(cipher, ciphertext.data(),
	                          pwm::crypto::DEFAULT_IV_SIZE_OCTETS));

	pwm::crypto::checkReturn(gcry_cipher_setkey(cipher, key.getKey().data(),
	                                            key.getKey().size()));

	std::vector<uint8_t> plaintext(
	        ciphertext.size() - pwm::crypto::DEFAULT_IV_SIZE_OCTETS, 0);
	pwm::crypto::checkReturn(gcry_cipher_decrypt(
	        cipher, plaintext.data(), plaintext.size(),
	        ciphertext.data() + pwm::crypto::DEFAULT_IV_SIZE_OCTETS,
	        ciphertext.size() - pwm::crypto::DEFAULT_IV_SIZE_OCTETS));

	return plaintext;
}
}

namespace pwm
{
namespace crypto
{
std::vector<uint8_t> decrypt(Key const &key,
                             std::vector<uint8_t> const &ciphertext)
{
	return decryptImpl(key, GCRY_CIPHER_SERPENT256,
	                   decryptImpl(key, GCRY_CIPHER_AES256, ciphertext));
}
}
}
