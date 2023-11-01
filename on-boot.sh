if [ ! -z $REPL_LANGUAGE ] && [ $REPL_LANGUAGE == 'python' ]
then
	ln -s .tutorial-python/ .tutorial
else
	ln -s .tutorial-rust/ .tutorial
fi
