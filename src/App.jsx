import React, { useState } from "react";
import { FaCopy, FaRedo } from "react-icons/fa";

function App() {
  const [password, setPassword] = useState("");
  const [includeUppercase, setIncludeUppercase] = useState(false);
  const [includeSpecialChars, setIncludeSpecialChars] = useState(false);
  const [length, setLength] = useState(10);
  const [numPasswords, setNumPasswords] = useState(1);

  const fetchPassword = () => {
    let url = `http://localhost:8000/api/password/${length}?`;

    if (includeUppercase) {
      url += "includeUppercase=true&";
    }

    if (includeSpecialChars) {
      url += "includeSpecialChars=true&";
    }

    url += `numPasswords=${numPasswords}&`;

    fetch(url)
      .then((response) => response.text())
      .then((data) => setPassword(data))
      .catch((error) => console.log(error));
  };

  const handleLengthChange = (e) => {
    const newLength = parseInt(e.target.value);
    if (newLength >= 5 && newLength <= 15) {
      setLength(newLength);
    }
  };

  const handleNumPasswordsChange = (e) => {
    const newNum = parseInt(e.target.value);
    if (newNum >= 1 && newNum <= 10) {
      setNumPasswords(newNum);
    }
  };

  const selectText = (event) => {
    const element = event.target;
    if (window.getSelection) {
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(element);
      selection.removeAllRanges();
      selection.addRange(range);
    } else if (document.body.createTextRange) {
      // IE <=8
      const range = document.body.createTextRange();
      range.moveToElementText(element);
      range.select();
    }
  };

  const copyToClipboard = (pwd) => {
    navigator.clipboard.writeText(pwd);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <div className="rounded-lg bg-white shadow-lg p-6 text-center">
        <h1 className="text-2xl sm:text-4xl font-bold mb-4">
          Générer mot de passe :
        </h1>

        <div className="flex flex-col space-y-3">
          <label className="flex items-center">
            <input
              type="checkbox"
              className="form-checkbox text-blue-500 rounded mr-2"
              checked={includeUppercase}
              onChange={() => setIncludeUppercase(!includeUppercase)}
            />
            <span className="text-lg text-gray-700">
              Inclure des majuscules
            </span>
          </label>
          <label className="flex items-center">
            <input
              type="checkbox"
              className="form-checkbox text-blue-500 rounded mr-2"
              checked={includeSpecialChars}
              onChange={() => setIncludeSpecialChars(!includeSpecialChars)}
            />
            <span className="text-lg text-gray-700">
              Inclure des caractères spéciaux
            </span>
          </label>

          <div className="flex items-center mb-4">
            <label
              htmlFor="numPasswords"
              className="text-lg text-gray-700 mr-2"
            >
              Nombre de mots de passe :
            </label>
            <select
              id="numPasswords"
              value={numPasswords}
              onChange={handleNumPasswordsChange}
              className="border border-gray-400 rounded px-2 py-1"
            >
              <option value="1">1</option>
              <option value="2">2</option>
              <option value="3">3</option>
              <option value="4">4</option>
              <option value="5">5</option>
              <option value="6">6</option>
              <option value="7">7</option>
              <option value="8">8</option>
              <option value="9">9</option>
              <option value="10">10</option>
            </select>
          </div>

          <div className="flex items-center mb-4">
            <label htmlFor="length" className="text-lg text-gray-700 mr-2">
              Longueur du mot de passe :
            </label>
            <select
              id="length"
              value={length}
              onChange={handleLengthChange}
              className="border border-gray-400 rounded px-2 py-1"
            >
              <option value="5">5</option>
              <option value="6">6</option>
              <option value="7">7</option>
              <option value="8">8</option>
              <option value="9">9</option>
              <option value="10">10</option>
              <option value="11">11</option>
              <option value="12">12</option>
              <option value="13">13</option>
              <option value="14">14</option>
              <option value="15">15</option>
            </select>
          </div>

          <div className="flex flex-col justify-center items-center">
            <button
              className="bg-blue-500 hover:bg-blue-700 text-lg text-white font-bold py-2 px-4 rounded w-60 sm:w-auto"
              onClick={fetchPassword}
            >
              Générer le mot de passe
            </button>

            {password && (
              <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 justify-center mt-4">
                {password.split("\n").map((pwd, idx) => (
                  <div
                    key={idx}
                    className="flex items-center space-x-2 bg-gray-200 p-4 rounded-lg shadow-md"
                  >
                    <p
                      className="text-2xl text-center flex-grow"
                      onClick={selectText}
                    >
                      {pwd}
                    </p>
                    <FaCopy
                      onClick={() => copyToClipboard(pwd)}
                      className="cursor-pointer text-blue-500 hover:text-blue-700 text-2xl mt-1"
                    />
                    <FaRedo
                      onClick={fetchPassword}
                      className="cursor-pointer text-blue-500 hover:text-blue-700 text-2xl mt-1"
                    />
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
