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

#include "ConfigurationRadixTree.hpp"

namespace
{
pwm::config::detail::ConfigurationRadixTreeNode *
getChild(pwm::config::detail::ConfigurationRadixTreeNode *parent,
         const std::string &k)
{
	for(const auto &child : parent->children)
	{
		if(child->key == k) return child.get();
	}

	parent->children.emplace_back(
	        new pwm::config::detail::ConfigurationRadixTreeNode(k));
	return parent->children.back().get();
}

void insert(pwm::config::detail::ConfigurationRadixTreeNode *root,
            const pwm::config::Key &key, const std::string &value)
{
	pwm::config::detail::ConfigurationRadixTreeNode *parent = root;
	for(const auto &component : key.components)
		parent = getChild(parent, component);
	parent->value = value;
}
}

namespace pwm
{
namespace config
{
namespace detail
{
ConfigurationRadixTreeNode::ConfigurationRadixTreeNode(const std::string &k)
        : key(k), children(), value()
{
}
}

ConfigurationRadixTree::ConfigurationRadixTree(const ConfigurationData &data)
        : root(new detail::ConfigurationRadixTreeNode(""))
{
	for(const auto &kv : data.data)
		insert(root.get(), kv.first, kv.second);
}

void ConfigurationRadixTree::traverse(const TraversalFunction &pre,
                                      const TraversalFunction &post) const
{
	std::function<void(const detail::ConfigurationRadixTreeNode &)> fn =
	        [this, &pre, &post, &fn](
	                const detail::ConfigurationRadixTreeNode &n)
	{
		if(&n != root.get())
		{
			if(pre && !pre(n.key, n.value)) return;
		}

		for(const auto &child : n.children)
			fn(*child);

		if(&n != root.get())
		{
			if(post && !post(n.key, n.value)) return;
		}
	};

	fn(*root);
}
}
}
