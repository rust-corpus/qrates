import plotly.graph_objects as go
from sklearn.tree import _tree

def draw_sankey_decision_tree(tree, feature_names):
    tree_ = tree.tree_
    feature_name = [
        feature_names[i] if i != _tree.TREE_UNDEFINED else "undefined!"
        for i in tree_.feature
    ]

    edges = []
    labels = {}
    
    def recurse(node, parent=None):
        if tree_.feature[node] != _tree.TREE_UNDEFINED:
            name = feature_name[node]
            threshold = tree_.threshold[node]
            
            right_child = tree_.children_right[node]
            right_size = recurse(right_child)
            edges.append( (node, right_child, right_size) )
            labels[right_child] = "{}".format(name)

            left_child = tree_.children_left[node]
            left_size = recurse(left_child)
            edges.append( (node, left_child, left_size) )
            labels[left_child] = "!{}".format(name)
        return tree_.n_node_samples[node]

    tot_size = recurse(0)
    labels[0] = "all"

    fig = go.Figure(data=[go.Sankey(
        arrangement = "freeform",
        orientation = "v",
        node = dict(
            #pad = 15,
            #thickness = 20,
            #line = dict(color = "black", width = 0.5),
            label = [labels[i] for i in range(max(labels.keys()) + 1)],
            color = ["red" if "!" in labels[i] else "green" for i in range(max(labels.keys()) + 1)],
        ),
        link = dict(
            # values in `source` and `target` correspond to label indices
            source = [x[0] for x in edges],
            target = [x[1] for x in edges],
            # amount of flow
            value  = [x[2] for x in edges],
        )
    )])
    fig.update_layout(title_text="Reasons of unsafety", font_size=10)
    fig.show()