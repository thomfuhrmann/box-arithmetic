import { useTiptap } from '@tiptap/react'

function MenuBar() {
    const { editor } = useTiptap()

    if (!editor) return null

    return (
        <div className="menu-bar">
            <button
                onClick={() => editor.chain().focus().toggleBold().run()}
                className={editor.isActive('bold') ? 'is-active' : ''}
            >
                Bold
            </button>
            <button
                onClick={() => editor.chain().focus().toggleItalic().run()}
                className={editor.isActive('italic') ? 'is-active' : ''}
            >
                Italic
            </button>
        </div>
    )
}

export default MenuBar;