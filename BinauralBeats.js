import React, { useState, useRef } from 'react';
import ReactPlayer from 'react-player';

const categories = [
  'Relax',
  'Focus',
  'Energize',
  'Sleep',
  'Meditate'
];

const BinauralBeats = () => {
  const [musicUrl, setMusicUrl] = useState('');
  const playerRef = useRef(null);

  const handleCategoryClick = async (category) => {
    // Fetch the music URL from the backend
    const response = await fetch(`http://127.0.0.1:8080/api/music/${category}`);
    const data = await response.json();
    setMusicUrl(data.url);
  };

  const handleStopMusic = () => {
    // Stop the music and clear the URL
    if (playerRef.current) {
      playerRef.current.seekTo(0); // Optional: Seek to the start
      playerRef.current.pause(); // Pause the playback
    }
    setMusicUrl('');
  };

  return (
    <div className="binaural-beats">
      <h2>Binaural Beats</h2>
      <div className="categories">
        {categories.map((category) => (
          <button
            key={category}
            onClick={() => handleCategoryClick(category)}
          >
            {category}
          </button>
        ))}
      </div>
      {musicUrl && (
        <div>
          <ReactPlayer
            ref={playerRef}
            url={musicUrl}
            playing
            controls
          />
          <button onClick={handleStopMusic}>Stop Music</button>
        </div>
      )}
    </div>
  );
};

export default BinauralBeats;
