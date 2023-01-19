export default (time: number) => {
	const minutes = Math.floor(time / 60);

	const seconds = time - minutes * 60;

	return `${(minutes + '').padStart(2, '0')}:${(seconds + '').padStart(2, '0')}`;
};
