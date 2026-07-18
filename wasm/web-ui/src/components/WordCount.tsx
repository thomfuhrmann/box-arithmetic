import { useTiptap, useTiptapState } from '@tiptap/react'

function WordCount() {
    const { editor } = useTiptap()

    const wordCount = useTiptapState((state) => {
        const text = state.editor.state.doc.textContent
        return text.split(/\s+/).filter(Boolean).length
    })

    if (!editor) {
        return null
    }

    return <span>{wordCount} words</span>
}

export default WordCount;