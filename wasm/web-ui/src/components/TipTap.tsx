"use client"

import { Tiptap, useEditor } from '@tiptap/react'
import { Extension, textInputRule } from '@tiptap/core'
import StarterKit from '@tiptap/starter-kit'
import { Button } from './ui/button'
import { ButtonGroup } from './ui/button-group'
import Color from '@tiptap/extension-color'
import { TextStyle } from '@tiptap/extension-text-style'
import { Separator } from './ui/separator'

const COLOR_RED = '#ff0000'

const MathSymbols = Extension.create({
    name: 'mathSymbols',

    addInputRules() {
        return [
            // Typing \lfloor inserts ⌊
            textInputRule({ find: /\\lfloor\s$/, replace: '⌊' }),
            // Typing \rfloor inserts ⌋
            textInputRule({ find: /\\rfloor\s$/, replace: '⌋' }),
            // Typing \lceil inserts ⌈
            textInputRule({ find: /\\lceil\s$/, replace: '⌈' }),
            // Typing \rceil inserts ⌉
            textInputRule({ find: /\\rceil\s$/, replace: '⌉' }),
        ]
    },
})

function Editor() {
    const editor = useEditor({
        extensions: [StarterKit, MathSymbols, TextStyle, Color],
        content: '',
        autofocus: true
    })

    if (!editor) return null

    // Helper to insert characters at the current cursor position
    const insertSymbol = (symbol: string, color = 'black') => {
        editor.chain().focus().setColor(color).insertContent(symbol).unsetColor().run()
    }

    return (
        <Tiptap editor={editor}>

            <div className="flex flex-wrap items-center gap-2 md:flex-row pl-4 pt-4">
                <ButtonGroup>
                    <Button variant="outline" onClick={() => insertSymbol('⌊')} title='open box'>⌊</Button>
                    <Button variant="outline" onClick={() => insertSymbol('⌋')} title="close box">⌋</Button>
                </ButtonGroup>
                <ButtonGroup>
                    <Button variant="outline" onClick={() => insertSymbol('⌈')} title="open list">⌈</Button>
                    <Button variant="outline" onClick={() => insertSymbol('⌉')} title="close list">⌉</Button>
                </ButtonGroup>
                <ButtonGroup>
                    <Button variant="outline" onClick={() => insertSymbol('[')} title="open maxel/vexel/pixel/unixel">{"["}</Button>
                    <Button variant="outline" onClick={() => insertSymbol(']')} title="close maxel/vexel/pixel/unixel">{"]"}</Button>
                </ButtonGroup>
                <ButtonGroup>
                    <Button variant="outline" onClick={() => insertSymbol('{')} title="open set">{"{"}</Button>
                    < Button variant="outline" onClick={() => insertSymbol('}')} title="close set">{"}"}</Button>
                </ButtonGroup>
                <Separator orientation="vertical" />
                <ButtonGroup>
                    <Button variant="outline" onClick={() => insertSymbol('{', COLOR_RED)} title="open set" className="text-red-500">{"{"}</Button>
                    < Button variant="outline" onClick={() => insertSymbol('}', COLOR_RED)} title="close set" className="text-red-500">{"}"}</Button>
                </ButtonGroup>
            </div>
            <div className='ml-4 mr-4 mt-4 border border-neutral-400 rounded-md'>
                <Tiptap.Content />
            </div>

        </Tiptap>
    )
}

export default Editor