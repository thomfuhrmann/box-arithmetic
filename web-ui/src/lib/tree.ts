import * as d3 from "d3";

type Color = "Red" | "Black";

export interface TreeNode {
	color: Color;
	multiplicity: string;
	children?: TreeNode[];
}

export function graph(
	container: HTMLElement,
	data: TreeNode,
	label = (d: { data: TreeNode }) => d.data.multiplicity,
	highlight = (d: { data: TreeNode }) => d.data.color === "Red",
	marginLeft = 40,
) {
	const root = d3.hierarchy(data);

	const dx = 50; // vertical distance between levels
	const dy = 80; // horizontal distance between nodes

	const tree = d3.tree<TreeNode>().nodeSize([dy, dx]);

	tree(root);

	// Determine horizontal extent of the tree.
	let x0 = Infinity;
	let x1 = -Infinity;
	root.each((d) => {
		if (d.x != null) {
			if (d.x < x0) x0 = d.x;
			if (d.x > x1) x1 = d.x;
		}
	});

	const width = x1 - x0 + marginLeft * 2;
	const height = (root.height + 1) * dx + 20;

	const svg = d3
		.create("svg")
		.attr("width", width)
		.attr("height", height)
		.style("overflow", "auto");

	const g = svg
		.append("g")
		.attr("font-family", "sans-serif")
		.attr("font-size", 10)
		.attr("transform", `translate(${marginLeft - x0}, 10)`);

	const link = d3
		.linkVertical<d3.HierarchyLink<TreeNode>, d3.HierarchyNode<TreeNode>>()
		.x((d) => d.x ?? 0)
		.y((d) => d.y ?? 0);

	// Links
	g.append("g")
		.attr("fill", "none")
		.attr("stroke", "#555")
		.attr("stroke-opacity", 0.4)
		.attr("stroke-width", 1.5)
		.selectAll("path")
		.data(root.links())
		.join("path")
		.attr("d", link);

	// Nodes
	const node = g
		.append("g")
		.attr("stroke-linejoin", "round")
		.attr("stroke-width", 3)
		.selectAll<SVGGElement, d3.HierarchyPointNode<TreeNode>>("g")
		.data(root.descendants())
		.join("g")
		.attr("transform", (d) => `translate(${d.x},${d.y})`);

	node
		.append("circle")
		.attr("fill", (d) => (highlight(d) ? "red" : d.children ? "#555" : "#999"))
		.attr("r", 2.5);

	node
		.append("text")
		.attr("stroke", "white")
		.attr("paint-order", "stroke")
		.attr("dy", "0.31em")
		.attr("x", () => -6)
		.attr("y", () => 6)
		.attr("text-anchor", "middle")
		.text(label);

	const svgNode = svg.node();
	if (svgNode) {
		container.replaceChildren(svgNode);
	}
}
