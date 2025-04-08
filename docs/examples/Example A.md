



```
Instrument instrument_xyz:
	type: Strings
	midi_path: xyz

Pattern intro(): # like a function
	
	return [1:8] Note(Am)+ [1:8] Note(A) + [1:8] Note(G) + [5:8] Wait()

Section Intro:
	Channel name_a:
		intro()
		
	Channel name_b:
		

	Channel name_c:

Song HotlineBling: 
	return Intro()

```
























