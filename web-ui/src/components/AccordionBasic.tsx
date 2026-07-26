import { ChevronRightIcon } from "lucide-react";
import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@/components/ui/accordion";

const items = [
	{
		value: "item-1",
		trigger: "Background",
		content: (
			<div>
				Box Arithmetic is a new approach to arithmetic and much of the
				mathematics built upon it, currently being developed by Norman J.
				Wildberger. Two key ideas form its foundation:
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							The most fundamental data structure is neither a set nor a list,
							but a <em>multiset</em>, whose elements are unordered and may
							occur multiple times.
						</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							The particle–antiparticle duality discovered by Paul Dirac has a
							deep and surprising analogue in the foundations of arithmetic.
						</span>
					</li>
				</ul>
				Combining these two ideas gives rise to Box Arithmetic.
			</div>
		),
	},
	{
		value: "item-2",
		trigger: "Syntax rules",
		content: (
			<div>
				Expressions entered into the calculator are interpreted using structural
				type inference. In other words, the structure of an expression
				determines its mathematical type and therefore the operations that apply
				to it. For example, a box containing 2-lists is interpreted as a maxel,
				and multiplication is performed as maxel multiplication.
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							Every opening bracket must have a matching closing bracket.
						</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Operators must be written explicitly.</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							Type{" "}
							<code className="bg-muted px-1 rounded font-mono">der(expr)</code>{" "}
							for derivatives of polynumbers
						</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							Type{" "}
							<code className="bg-muted px-1 rounded font-mono">
								der(expr, n)
							</code>{" "}
							for derivatives of multinumbers, where{" "}
							<code className="bg-muted px-1 rounded font-mono">n</code>{" "}
							specifies the index of β
						</span>
					</li>
				</ul>
			</div>
		),
	},
	{
		value: "item-3",
		trigger: "Input and output formats",
		content: (
			<div>
				The calculator supports two primary display formats.
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							<strong>Mixed format:</strong> Expressions are displayed in
							traditional mathematical notation whenever possible. Some
							expressions cannot be represented completely in classical notation
							because they have no equivalent in the traditional mathematical
							framework.
						</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							<strong>Box format:</strong> Expressions are displayed directly in
							their box representation without being translated into traditional
							mathematical notation. Nested structures are shown simply as boxes
							within boxes.
						</span>
					</li>
				</ul>
				Both formats are available in expanded and compact forms. In the
				expanded form, every nested object is written out explicitly each time
				it occurs. In the compact form, repeated objects are represented by a
				subscript indicating their multiplicity.
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Mixed, expanded: ⌊⌈1,1⌉,⌈1,2⌉,⌈2,2⌉,⌈2,2⌉⌋</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Mixed, compact: ⌊⌈1,1⌉,⌈1,2⌉,₂⌈2,2⌉⌋</span>
					</li>
				</ul>
			</div>
		),
	},
	{
		value: "item-4",
		trigger: "Supported objects and operations",
		content: (
			<div>
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>
							Supported objects: numbers, polynumbers, unixels, pixels, maxels,
							vexels, sets, and lists.
						</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Supported operators: +, -, *, ∪, ∩.</span>
					</li>
				</ul>
			</div>
		),
	},
	{
		value: "item-5",
		trigger: "Known limitations",
		content: (
			<div>
				This project is still in an early stage of development, and many
				features are not yet complete. Bugs may still exist. If you encounter
				one, please consider opening an issue or submitting a pull request on
				the GitHub repository linked above.
				<p className="mt-2">
					The following features are planned but not yet fully implemented:
				</p>
				<ul className="grid gap-2 py-2 text-sm">
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Division of boxes.</span>
					</li>
					<li className="flex gap-2">
						<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
						<span>Saving expressions and assigning names to them.</span>
					</li>
				</ul>
			</div>
		),
	},
];
export function AccordionBasic() {
	return (
		<Accordion defaultValue={["item-1"]} multiple className="px-16">
			{items.map((item) => (
				<AccordionItem key={item.value} value={item.value}>
					<AccordionTrigger>{item.trigger}</AccordionTrigger>
					<AccordionContent>{item.content}</AccordionContent>
				</AccordionItem>
			))}
		</Accordion>
	);
}
