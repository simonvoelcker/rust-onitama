import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Cell extends Component {
  constructor (props) {
    super(props)
    this.state = {
    }
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='cell'>
            <span>{this.props.content}</span>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Cell.contextType = AppContext
