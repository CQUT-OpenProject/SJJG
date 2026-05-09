void inorder(blink bt)
{
    if(bt)
    {
        inorder(bt->lchild);
        printf("%c", bt->data);
        inorder(bt->rchild);
    }
}

// 最终输出：abehmpqs