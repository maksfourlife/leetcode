// https://leetcode.com/problems/copy-list-with-random-pointer

#include <unordered_map>

class Node
{
public:
    int val;
    Node *next;
    Node *random;

    Node(int _val)
    {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};

Node *copyRandomList(Node *head)
{
    std::unordered_map<Node *, Node *> cloned_nodes;
    f(head, cloned_nodes);
}

Node *f(Node *node, std::unordered_map<Node *, Node *> &cloned_nodes)
{
    if (node == nullptr)
    {
        return nullptr;
    }
    auto cloned_node_it = cloned_nodes.find(node);
    if (cloned_node_it != cloned_nodes.end())
    {
        return cloned_node_it->second;
    }
    auto cloned_node = new Node(node->val);
    cloned_nodes.insert({node, cloned_node});
    cloned_node->next = f(node->next, cloned_nodes);
    cloned_node->random = f(node->random, cloned_nodes);
    return cloned_node;
}

int main()
{
}