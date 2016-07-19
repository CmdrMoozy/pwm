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

#include "encrypt.hpp"

#include <cstddef>

#include <gcrypt.h>

#include <bdrck/util/ScopeExit.hpp>

#include "pwmc/crypto/Key.hpp"
#include "pwmc/crypto/Padding.hpp"
#include "pwmc/crypto/Util.hpp"
#include "pwmc/crypto/checkReturn.hpp"
#include "pwmc/util/MemoryFile.hpp"

namespace
{
std::vector<uint8_t> encryptImpl(pwm::crypto::Key const &key, int algorithm,
                                 uint8_t const *plaintext, std::size_t size)
{
	// Pad the input data.
	std::vector<uint8_t> paddedPlaintext(plaintext, plaintext + size);
	pwm::crypto::padding::pad(paddedPlaintext, algorithm);

	// Initialize the cipher.
	gcry_cipher_hd_t cipher;
	pwm::crypto::checkReturn(gcry_cipher_open(
	        &cipher, algorithm, GCRY_CIPHER_MODE_CBC, GCRY_CIPHER_SECURE));
	bdrck::util::ScopeExit destroyCipher(
	        [&cipher]() { gcry_cipher_close(cipher); });

	std::vector<uint8_t> ciphertext(paddedPlaintext.size(), 0);

	// Setup the cipher's initialization vector, adding it to ciphertext.
	std::vector<uint8_t> iv = pwm::crypto::util::generateRandomBytes(
	        pwm::crypto::DEFAULT_IV_SIZE_OCTETS,
	        pwm::crypto::util::RandomQuality::VERY_STRONG);
	pwm::crypto::checkReturn(
	        gcry_cipher_setiv(cipher, iv.data(), iv.size()));
	ciphertext.insert(ciphertext.end(), iv.data(), iv.data() + iv.size());

	// Set the encryption key.
	pwm::crypto::checkReturn(gcry_cipher_setkey(cipher, key.getKey().data(),
	                                            key.getKey().size()));

	// Encrypt!
	pwm::crypto::checkReturn(gcry_cipher_encrypt(
	        cipher, ciphertext.data(), paddedPlaintext.size(),
	        paddedPlaintext.data(), paddedPlaintext.size()));
	return ciphertext;
}

std::vector<uint8_t> encryptImpl(pwm::crypto::Key const &key, int algorithm,
                                 std::vector<uint8_t> const &plaintext)
{
	return encryptImpl(key, algorithm, plaintext.data(), plaintext.size());
}
}

namespace pwm
{
namespace crypto
{
std::vector<uint8_t> encrypt(Key const &key, uint8_t const *plaintext,
                             std::size_t size)
{
	/*
	 * For maximum protection, we're going to encrypt the plaintext with
	 * Serpent 256, and then with AES 256. We use random IVs, and simply
	 * prepend the IV to the ciphertext. Start by encrypting with Serpent:
	 */

	return encryptImpl(
	        key, GCRY_CIPHER_AES256,
	        encryptImpl(key, GCRY_CIPHER_SERPENT256, plaintext, size));
}

std::vector<uint8_t> encrypt(Key const &key,
                             std::vector<uint8_t> const &plaintext)
{
	return encrypt(key, plaintext.data(), plaintext.size());
}
}
}
