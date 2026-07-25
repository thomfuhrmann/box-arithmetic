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
import parse from "html-react-parser";
import { CornerDownLeftIcon, GitBranchIcon, Subscript } from "lucide-react";
import { useMemo, useState } from "react";
import { BoxCalculator } from "wasm";
import { AccordionBasic } from "./AccordionBasic";
import { Button } from "./ui/button";
import { ButtonGroup } from "./ui/button-group";
import {
	Card,
	CardAction,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "./ui/card";
import { Marker } from "./ui/marker";
import { Separator } from "./ui/separator";

const COLOR_RED = "#ff0000";

interface EvalOutput {
	mixed: string;
	mixed_mul: string;
	boxed: string;
	boxed_mul: string;
}

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

export interface ShiftEnterExtractorOptions {
	calculator?: BoxCalculator | null | undefined;
	onEvaluate?: (result: EvalOutput | null) => void;
	onError?: (error: unknown) => void;
}

// Extension to capture Shift+Enter and extract text
const ShiftEnterExtractor = Extension.create<
	ShiftEnterExtractorOptions,
	unknown
>({
	name: "shiftEnterExtractor",

	addOptions() {
		return {
			calculator: null,
			onEvaluate: undefined,
			onError: undefined,
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
				const calculator = this.options.calculator;

				if (!calculator) {
					console.warn(
						"BoxCalculator instance not supplied to ShiftEnterExtractor",
					);
					return true;
				}

				if (this.options.onError) {
					this.options.onError(null);
				}
				if (this.options.onEvaluate) {
					this.options.onEvaluate(null);
				}

				try {
					const outputExpr: EvalOutput = calculator.eval_expr(inputExpr);

					const formatRedTags = (str: string): string => {
						if (!str) return "";
						return str
							.replaceAll("<red>", "<span style='color: rgb(255, 0, 0);'>")
							.replaceAll("</red>", "</span>");
					};
					const formattedOutput: EvalOutput = {
						mixed: formatRedTags(outputExpr.mixed),
						mixed_mul: formatRedTags(outputExpr.mixed_mul),
						boxed: formatRedTags(outputExpr.boxed),
						boxed_mul: formatRedTags(outputExpr.boxed_mul),
					};

					if (this.options.onEvaluate) {
						this.options.onEvaluate(formattedOutput);
					}
				} catch (e) {
					if (this.options.onError) {
						this.options.onError(e);
					}
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

// Extension that converts input into anti-expression
export const AntiRule = Extension.create({
	name: "antiRule",

	addInputRules() {
		return [
			new InputRule({
				// Matches \anti followed by non-whitespace characters, completed by typing a space
				find: /\\anti\s+([^\s]+)\s$/,
				handler: ({ state, range, match }) => {
					const [, content] = match;
					const { from, to } = range;

					// Retrieve or resolve the textStyle mark type
					const textStyleType = state.schema.marks.textStyle;

					if (!textStyleType) {
						console.warn("TextStyle extension is required for AntiRule");
						return;
					}

					const redMark = textStyleType.create({ color: COLOR_RED });

					state.tr
						.insertText(`${content} `, from, to)
						.addMark(from, from + content.length, redMark)
						.removeStoredMark(redMark.type);
				},
			}),
		];
	},
});

function Editor() {
	const [evalResult, setEvalResult] = useState<EvalOutput | null>(null);
	const [errorResult, setErrorResult] = useState<string | null>(null);
	const calculator = useMemo(() => new BoxCalculator(), []);

	const editor = useEditor({
		extensions: [
			StarterKit,
			MathSymbols,
			TextStyle,
			Color,
			ShiftEnterExtractor.configure({
				calculator,
				onEvaluate: (result) => {
					setEvalResult(result);
				},
				onError: (e) => {
					if (e !== null) {
						setErrorResult(String(e));
					} else {
						setErrorResult(null);
					}
				},
			}),
			UnicodeSubscript,
			AntiRule,
		],
		content: "",
		autofocus: true,
		editorProps: {
			attributes: {
				class: "focus:outline-none min-h-[200px] h-full",
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
		<div className="container mx-auto max-w-5xl p-6 space-y-6">
			<Card>
				<CardHeader>
					<CardAction>
						<Marker
							render={
								<a href="https://github.com/thomfuhrmann/box-arithmetic" />
							}
						>
							<GitBranchIcon></GitBranchIcon>GitHub repository
						</Marker>
					</CardAction>
					<CardTitle>Box Calculator</CardTitle>
					<CardDescription>A Calculator for Box Arithmetic</CardDescription>
				</CardHeader>
				<CardContent>
					<AccordionBasic></AccordionBasic>
				</CardContent>
			</Card>

			<div className="space-y-4">
				<div className="flex items-center justify-between">
					<span className="text-sm font-semibold tracking-wide uppercase text-muted-foreground">
						Expression Editor
					</span>
					<span className="text-xs text-muted-foreground flex items-center gap-1">
						Press{" "}
						<kbd className="px-1.5 py-0.5 rounded bg-muted border font-mono text-[10px]">
							Shift
						</kbd>{" "}
						+{" "}
						<kbd className="px-1.5 py-0.5 rounded bg-muted border font-mono text-[10px]">
							Enter
						</kbd>{" "}
						to run
					</span>
				</div>

				<Tiptap editor={editor}>
					<Card className="overflow-hidden border shadow-sm focus-within:border-primary/50 transition-colors pt-0">
						{/* Math Symbol Toolbar */}
						<div className="flex flex-wrap items-center gap-1.5 border-b bg-muted/30 p-4 text-xs">
							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("□")}
									title="empty box"
								>
									□
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("□", COLOR_RED)}
									title="red empty box"
									className="text-red-500"
								>
									□
								</Button>
							</ButtonGroup>

							<Separator orientation="vertical" className="mx-2" />

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌊")}
									title="open box"
								>
									⌊
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌋")}
									title="close box"
								>
									⌋
								</Button>
							</ButtonGroup>

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌈")}
									title="open list"
								>
									⌈
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌉")}
									title="close list"
								>
									⌉
								</Button>
							</ButtonGroup>

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("{")}
									title="open set"
								>
									{"{"}
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("}")}
									title="close set"
								>
									{"}"}
								</Button>
							</ButtonGroup>

							<Separator orientation="vertical" className="mx-2" />

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌊", COLOR_RED)}
									title="red open box"
									className="text-red-500"
								>
									⌊
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌋", COLOR_RED)}
									title="red close box"
									className="text-red-500"
								>
									⌋
								</Button>
							</ButtonGroup>

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌈", COLOR_RED)}
									title="red open list"
									className="text-red-500"
								>
									⌈
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("⌉", COLOR_RED)}
									title="red close list"
									className="text-red-500"
								>
									⌉
								</Button>
							</ButtonGroup>

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("{", COLOR_RED)}
									title="red open set"
									className="text-red-500"
								>
									{"{"}
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("}", COLOR_RED)}
									title="red close set"
									className="text-red-500"
								>
									{"}"}
								</Button>
							</ButtonGroup>

							<Separator orientation="vertical" className="mx-2" />

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("α")}
									title="alpha"
								>
									{"α"}
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("α", COLOR_RED)}
									title="red alpha"
									className="text-red-500"
								>
									{"α"}
								</Button>
							</ButtonGroup>

							<Separator orientation="vertical" className="mx-2" />

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("∪")}
									title="union"
								>
									{"∪"}
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("∩")}
									title="intersection"
								>
									{"∩"}
								</Button>
							</ButtonGroup>

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("∪", COLOR_RED)}
									title="red union"
									className="text-red-500"
								>
									{"∪"}
								</Button>

								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("∩", COLOR_RED)}
									title="red intersection"
									className="text-red-500"
								>
									{"∩"}
								</Button>
							</ButtonGroup>

							<Separator orientation="vertical" className="mx-2" />

							<ButtonGroup>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("\\anti ")}
									title="anti"
									className="text-red-500"
								>
									{"anti"}
								</Button>
								<Button
									variant="outline"
									size="default"
									onClick={() => insertSymbol("_")}
									title="subscript"
								>
									<Subscript></Subscript>
								</Button>
							</ButtonGroup>
						</div>

						<CardContent>
							<div className="w-full bg-background min-h-40">
								<Tiptap.Content className="w-full h-full min-h-40" />
							</div>
						</CardContent>

						<CardFooter className="flex items-center justify-between bg-muted/10 border-t py-2 px-4 text-xs text-muted-foreground">
							<span>
								Type{" "}
								<code className="bg-muted px-1 rounded font-mono">\lbox</code>
								{", "}
								<code className="bg-muted px-1 rounded font-mono">\rbox</code>
								{", "}
								<code className="bg-muted px-1 rounded font-mono">\llist</code>
								{", "}
								<code className="bg-muted px-1 rounded font-mono">\rlist</code>{" "}
								for quick brackets;{" "}
								<code className="bg-muted px-1 rounded font-mono">
									_ + number
								</code>
								for subscripts;{" "}
								<code className="bg-muted px-1 rounded font-mono">\anti</code>
								to convert symbol into anti
							</span>
							<Button
								size="default"
								variant="secondary"
								className="h-7 gap-1 font-sans text-xs"
								onClick={() => {
									const event = new KeyboardEvent("keydown", {
										key: "Enter",
										shiftKey: true,
										bubbles: true,
									});
									editor.view.dom.dispatchEvent(event);
								}}
							>
								<CornerDownLeftIcon className="size-3" />
								Evaluate
							</Button>
						</CardFooter>
					</Card>
				</Tiptap>
			</div>

			{evalResult ? (
				<Card>
					<CardHeader>
						<CardTitle>Evaluation Result</CardTitle>
					</CardHeader>

					<CardContent className="space-y-6">
						<div>
							<h4 className="mb-2 text-sm font-medium">Mixed format</h4>
							<div>{parse(evalResult.mixed)}</div>
						</div>

						{evalResult.mixed !== evalResult.mixed_mul && (
							<>
								<Separator />
								<div>
									<h4 className="mb-2 text-sm font-medium">
										Mixed format (with multiplicities)
									</h4>
									<div>{parse(evalResult.mixed_mul)}</div>
								</div>
							</>
						)}

						<Separator />

						<div>
							<h4 className="mb-2 text-sm font-medium">Box format</h4>
							<div>{parse(evalResult.boxed)}</div>
						</div>

						{evalResult.boxed !== evalResult.boxed_mul && (
							<>
								<Separator />
								<div>
									<h4 className="mb-2 text-sm font-medium">
										Box format (with multiplicities)
									</h4>
									<div>{parse(evalResult.boxed_mul)}</div>
								</div>
							</>
						)}
					</CardContent>
				</Card>
			) : null}
			{errorResult !== null ? (
				<Card>
					<CardHeader>
						<CardTitle className="text-red-500">Evaluation Error</CardTitle>
					</CardHeader>

					<CardContent className="text-red-500">
						<div>
							<div>{errorResult}</div>
						</div>
					</CardContent>
				</Card>
			) : null}
		</div>
	);
}

export default Editor;
