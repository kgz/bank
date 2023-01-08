import React from 'react';
import { NavLink } from 'react-router-dom';
import { ctx } from '../..';


function Index() {

  const {isLoggedin, setIsLoggedin} = React.useContext(ctx);

  return (
    <div className="App">
      Home 

      <NavLink to="/migrations">Migrations</NavLink><br />
      <NavLink to="/login">Login</NavLink>
      <button onClick={() => setIsLoggedin(false)}>Logout</button>
    </div>
  );
}

export default Index;
