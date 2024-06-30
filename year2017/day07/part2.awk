#!/usr/bin/awk -f

BEGIN{FPAT="[a-z0-9]+"}
{ weight[$1] = $2 }
{
    starting_i = 3;
    for (i = starting_i; i <= NF; ++i) {
        parent[$i] = $1;
        children[$1][i - starting_i] = $i;
    }
}

function calculate_tree_weights(node, tree_weight) {
    tree_weight = weight[node];
    for (child_i in children[node]) {
        tree_weight += calculate_tree_weights(children[node][child_i]);
    }
    tree_weights[node] = tree_weight;
    return tree_weight;
}

# node and expected_weight are actual arguments, the rest is local variables
function find_correct_balance(node, expected_weight, child, children_weights, child_weight, child_weight_counter, total_children_weight, invalid_weight, valid_weight) {
    for (child_i in children[node]) {
        child = children[node][child_i];
        child_weight = tree_weights[child]
        children_weights[child] = child_weight;
        child_weight_counter[child_weight]++;
    }

    # all children weigh the same
    if (length(child_weight_counter) == 1) {
        return expected_weight - (child_weight * child_weight_counter[child_weight]);
    }

    if (length(child_weight_counter) != 2) {
        print "something's wrong";
        exit 1;
    }

    for (child_weight in child_weight_counter) {
        if (child_weight_counter[child_weight] == 1) {
            invalid_weight = child_weight;
        } else {
            valid_weight = child_weight;
        }
    }

    if (valid_weight <= 0 || invalid_weight <= 0) {
        print "something's wrong";
        exit 2;
    }

    for (child_i in children[node]) {
        child = children[node][child_i];
        if (tree_weights[child] == invalid_weight) {
            return find_correct_balance(child, valid_weight);
        }
    }
}

END{
    for (prog in weight) {
        if (!(prog in parent)) {
            root = prog;
            break;
        }
    }

    calculate_tree_weights(root);
    sol=find_correct_balance(root);
    print sol;
}
