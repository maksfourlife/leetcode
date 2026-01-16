// https://leetcode.com/problems/linked-list-cycle-ii/

#include <unordered_set>

struct ListNode
{
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

ListNode *detectCycle(ListNode *head)
{
    std::unordered_set<ListNode *> seen_nodes;

    auto curr = head;
    while (curr != nullptr)
    {
        if (seen_nodes.find(curr) != seen_nodes.end())
        {
            return curr;
        }
        seen_nodes.insert(curr);
        curr = curr->next;
    }

    return nullptr;
}

int main()
{
}