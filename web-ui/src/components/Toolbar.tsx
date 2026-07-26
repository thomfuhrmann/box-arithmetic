import { useTiptap } from "@tiptap/react";
import { Subscript } from "lucide-react";
import { COLOR_RED } from "./TipTap";
import { Button } from "./ui/button";
import { ButtonGroup } from "./ui/button-group";
import { Separator } from "./ui/separator";

function Toolbar() {
	const { editor } = useTiptap();

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

			<ButtonGroup>
				<Button
					variant="outline"
					size="default"
					onClick={() => insertSymbol("β_")}
					title="beta"
				>
					{"β"}
				</Button>

				<Button
					variant="outline"
					size="default"
					onClick={() => {
						insertSymbol("β", COLOR_RED);
						insertSymbol("_", COLOR_RED);
					}}
					title="red beta"
					className="text-red-500"
				>
					{"β"}
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
	);
}

export default Toolbar;
