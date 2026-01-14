// https://leetcode.com/problems/clone-graph

#include <vector>
#include <unordered_map>

class Node
{
public:
    int val;
    std::vector<Node *> neighbors;

    Node()
    {
        val = 0;
        neighbors = std::vector<Node *>();
    }

    Node(int _val)
    {
        val = _val;
        neighbors = std::vector<Node *>();
    }

    Node(int _val, std::vector<Node *> _neighbors)
    {
        val = _val;
        neighbors = _neighbors;
    }
};

Node *f(Node *node, std::unordered_map<Node *, Node *> &cloned_nodes);

Node *cloneGraph(Node *node)
{
    std::unordered_map<Node *, Node *> cloned_nodes;
    return f(node, cloned_nodes);
}

Node *f(Node *node, std::unordered_map<Node *, Node *> &cloned_nodes)
{
    auto maybe_cloned_node = cloned_nodes.find(node);
    if (maybe_cloned_node != cloned_nodes.end())
    {
        return maybe_cloned_node->second;
    }
    auto cloned_node = new Node(node->val);
    cloned_nodes.insert({node, cloned_node});
    for (auto *neighbour : node->neighbors)
    {
        cloned_node->neighbors.push_back(f(neighbour, cloned_nodes));
    }
    return cloned_node;
}

int main()
{
    auto node1 = new Node(1);
    auto node2 = new Node(2);
    auto node3 = new Node(3);
    auto node4 = new Node(4);
    node1->neighbors = {node2, node4};
    node2->neighbors = {node1, node3};
    node3->neighbors = {node2, node4};
    node4->neighbors = {node1, node3};
    cloneGraph(node1);
}