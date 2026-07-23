"use client";

import {
	Extension,
	InputRule,
	type JSONContent,
	textInputRule,
} from "@tiptap/core";
import Color from "@tiptap/extension-color";
import { TextStyle } from "@tiptap/extension-text-style";
import { Tiptap, useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import { ChevronRightIcon, GitBranchIcon } from "lucide-react";
import { useState } from "react";
import { BoxCalculator } from "wasm";
import { Button } from "./ui/button";
import { ButtonGroup } from "./ui/button-group";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "./ui/card";
import { Marker } from "./ui/marker";
import { Separator } from "./ui/separator";

const COLOR_RED = "#ff0000";

const MathSymbols = Extension.create({
	name: "mathSymbols",

	addInputRules() {
		return [
			textInputRule({ find: /\\lbox\s$/, replace: "⌊" }),
			textInputRule({ find: /\\rbox\s$/, replace: "⌋" }),
			textInputRule({ find: /\\llist\s$/, replace: "⌈" }),
			textInputRule({ find: /\\rlist\s$/, replace: "⌉" }),
		];
	},
});

interface OutputValue {
	mixed: string;
	mixed_mul: string;
	boxed: string;
	boxed_mul: string;
}

export interface ShiftEnterExtractorOptions {
	onEvaluate?: (result: OutputValue, inputExpr: string) => void;
}

// Extension to capture Shift+Enter and extract text
const ShiftEnterExtractor = Extension.create({
	name: "shiftEnterExtractor",

	addOptions() {
		return {
			onEvaluate: (_res: OutputValue) => {},
		};
	},

	addKeyboardShortcuts() {
		return {
			"Shift-Enter": ({ editor }) => {
				const docJson = editor.getJSON();

				// Recursive function to parse the document tree
				const serializeNode = (node: JSONContent): string => {
					if (node.type === "text") {
						const text = node.text || "";

						// Check if this text fragment has styling marks applied
						const colorMark = node.marks?.find((m) => m.type === "textStyle");

						if (colorMark?.attrs?.color === COLOR_RED) {
							// Wrap in a minimal indicator for red, like [r: symbol]
							return `<red>${text}</red>`;
						}
						return text;
					}

					if (node.content) {
						const contentString = node.content.map(serializeNode).join("");

						// Add block separation for paragraphs if there are multiple lines
						if (node.type === "paragraph") {
							return `${contentString}\n`;
						}
						return contentString;
					}

					return "";
				};

				// Generate token string and trim trailing newlines
				const inputExpr = serializeNode(docJson).trim();

				console.log("input expression:", inputExpr);

				const calculator = new BoxCalculator();

				try {
					const outputExpr = calculator.eval_expr(inputExpr);

					if (this.options.onEvaluate) {
						this.options.onEvaluate(outputExpr);
					}
				} catch (e) {
					console.log(e);
				}

				return true;
			},
		};
	},
});

const subscriptDigits: Record<string, string> = {
	"0": "₀",
	"1": "₁",
	"2": "₂",
	"3": "₃",
	"4": "₄",
	"5": "₅",
	"6": "₆",
	"7": "₇",
	"8": "₈",
	"9": "₉",
};

function toSubscript(s: string) {
	return s.replace(/\d/g, (d) => subscriptDigits[d]);
}

// Extension for input of subscripts
export const UnicodeSubscript = Extension.create({
	name: "unicodeSubscript",

	addInputRules() {
		return [
			new InputRule({
				find: /_(\d+)\s$/,
				handler: ({ state, range, match }) => {
					const [, digits] = match;

					state.tr.insertText(`${toSubscript(digits)} `, range.from, range.to);
				},
			}),
		];
	},
});

function Editor() {
	const [evalResult, setEvalResult] = useState<OutputValue | null>(null);

	const editor = useEditor({
		extensions: [
			StarterKit,
			MathSymbols,
			TextStyle,
			Color,
			ShiftEnterExtractor.configure({
				onEvaluate: (result: OutputValue) => {
					setEvalResult(result);
				},
			}),
			UnicodeSubscript,
		],
		content: "",
		autofocus: true,
		editorProps: {
			attributes: {
				class: "focus:outline-none min-h-[200px] h-full p-4",
			},
		},
	});

	if (!editor) return null;

	const insertSymbol = (symbol: string, color = "black") => {
		editor
			.chain()
			.focus()
			.setColor(color)
			.insertContent(symbol)
			.unsetColor()
			.run();
	};

	return (
		<div className="container mx-auto max-w-4xl p-6 space-y-6">
			<Card>
				<CardHeader>
					<CardTitle>Box Arithmetic Calculator</CardTitle>
					<CardDescription>
						Input Rules:
						<ul className="grid gap-2 py-2 text-sm">
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Press Shift + Enter to evaluate an expression</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Enter _ + number to insert a number in subscript</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Operators must be inserted explicitly</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Boxes must be separated by commas</span>
							</li>
						</ul>
						IO-Formats:
						<ul className="grid gap-2 py-2 text-sm">
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Classic: 1 + 2 * α ^ 2</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									Box: ⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉,⌈⌊□,□⌋,⌊□,□⌋⌉⌋
								</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Mixed: ⌊⌈1,1⌉,⌈1,2⌉,⌈2,2⌉,⌈2,2⌉⌋</span>
							</li>
						</ul>
						<p>
							Both, box and mixed, formats support a compact form using
							multiplicities and an expanded form, where every box is written
							down explicitly. Use subscripts to denote multiplcities:
							⌊⌈1,1⌉,⌈1,2⌉,₂⌈2,2⌉⌋
						</p>
						<br />
						Currently supported objects and operaters:
						<ul className="grid gap-2 py-2 text-sm">
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									Objects: numbers, polynumbers, unixels, pixels, maxels,
									vexels, sets and lists
								</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Operaters: +, -, *, ∪, ∩</span>
							</li>
						</ul>
						<br />
						Known limitations: The library is still at an early stage and there
						might be bugs. Feel free to open an issue in the GitHub repository
						linked at the bottom. Missing but planned features include:
						<ul className="grid gap-2 py-2 text-sm">
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>Derivatives of polynumber and multinumbers</span>
							</li>
							<li className="flex gap-2">
								<ChevronRightIcon className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
								<span>
									Support for storing expressions and defining the names of
									variables
								</span>
							</li>
						</ul>
					</CardDescription>
				</CardHeader>
			</Card>

			<Tiptap editor={editor}>
				<Card className="overflow-hidden border bg-card text-card-foreground shadow">
					{/* Toolbar Panel */}
					<div className="flex flex-wrap items-center gap-2 border-b bg-muted/40 p-3">
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("□")}
								title="empty box"
							>
								□
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("□", COLOR_RED)}
								title="empty red box"
								className="text-red-500"
							>
								□
							</Button>
						</ButtonGroup>

						<Separator orientation="vertical" className="h-6 mx-1" />

						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌊")}
								title="open box"
							>
								⌊
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌋")}
								title="close box"
							>
								⌋
							</Button>
						</ButtonGroup>
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌈")}
								title="open list"
							>
								⌈
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌉")}
								title="close list"
							>
								⌉
							</Button>
						</ButtonGroup>
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("{")}
								title="open set"
							>
								{"{"}
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("}")}
								title="close set"
							>
								{"}"}
							</Button>
						</ButtonGroup>

						<Separator orientation="vertical" className="h-6 mx-1" />

						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌊", COLOR_RED)}
								title="open box"
								className="text-red-500"
							>
								⌊
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌋", COLOR_RED)}
								title="close box"
								className="text-red-500"
							>
								⌋
							</Button>
						</ButtonGroup>
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌈", COLOR_RED)}
								title="open list"
								className="text-red-500"
							>
								⌈
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("⌉", COLOR_RED)}
								title="close list"
								className="text-red-500"
							>
								⌉
							</Button>
						</ButtonGroup>
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("{", COLOR_RED)}
								title="open set"
								className="text-red-500"
							>
								{"{"}
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("}", COLOR_RED)}
								title="close set"
								className="text-red-500"
							>
								{"}"}
							</Button>
						</ButtonGroup>

						<Separator orientation="vertical" className="h-6 mx-1" />

						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("α")}
								title="open set"
							>
								{"α"}
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("α", COLOR_RED)}
								title="close set"
								className="text-red-500"
							>
								{"α"}
							</Button>
						</ButtonGroup>

						<Separator orientation="vertical" className="h-6 mx-1" />

						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("∪")}
								title="union"
							>
								{"∪"}
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("∪", COLOR_RED)}
								title="union"
								className="text-red-500"
							>
								{"∪"}
							</Button>
						</ButtonGroup>
						<ButtonGroup>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("∩")}
								title="intersection"
							>
								{"∩"}
							</Button>
							<Button
								variant="outline"
								size="sm"
								onClick={() => insertSymbol("∩", COLOR_RED)}
								title="intersection"
								className="text-red-500"
							>
								{"∩"}
							</Button>
						</ButtonGroup>
					</div>

					{/* Editor Content Area */}
					<CardContent className="p-0">
						<div className="w-full h-full min-h-50 bg-background">
							<Tiptap.Content className="w-full h-full [&>.ProseMirror]:h-full [&>.ProseMirror]:min-h-50" />
						</div>
					</CardContent>
					<CardFooter className="flex-col gap-2">
						<Marker
							render={
								<a href="https://github.com/thomfuhrmann/box-arithmetic" />
							}
						>
							<GitBranchIcon></GitBranchIcon>GitHub repository
						</Marker>
					</CardFooter>
				</Card>
			</Tiptap>

			{evalResult ? (
				<Card>
					<CardHeader>Evaluation result</CardHeader>
					<CardContent>{evalResult.mixed}</CardContent>
					<CardContent>{evalResult.mixed_mul}</CardContent>
					<CardContent>{evalResult.boxed}</CardContent>
					<CardContent>{evalResult.boxed_mul}</CardContent>
				</Card>
			) : null}
		</div>
	);
}

export default Editor;
